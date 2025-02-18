// This file is part of CORD – https://cord.network

// Copyright (C) Dhiway Networks Pvt. Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later

// CORD is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// CORD is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with CORD. If not, see <https://www.gnu.org/licenses/>.

use super::*;
use crate::mock::*;
use codec::Encode;
use cord_utilities::mock::{mock_origin::DoubleOrigin, SubjectId};
use frame_support::{assert_ok, BoundedVec};
use frame_system::RawOrigin;
use pallet_chain_space::SpaceCodeOf;
use sp_runtime::{traits::Hash, AccountId32};
use sp_std::prelude::*;

pub fn generate_rating_id<T: Config>(digest: &RatingEntryHashOf<T>) -> RatingEntryIdOf {
	Ss58Identifier::create_identifier(&(digest).encode()[..], IdentifierType::Rating).unwrap()
}

pub fn generate_space_id<T: Config>(digest: &SpaceCodeOf<T>) -> SpaceIdOf {
	Ss58Identifier::create_identifier(&(digest).encode()[..], IdentifierType::Space).unwrap()
}

pub(crate) const DID_00: SubjectId = SubjectId(AccountId32::new([1u8; 32]));
pub(crate) const ACCOUNT_00: AccountId = AccountId::new([1u8; 32]);

#[test]
fn check_successful_rating_creation() {
	let creator = DID_00;
	let author = ACCOUNT_00;

	let message_id = BoundedVec::try_from([72u8; 10].to_vec()).unwrap();
	let entity_uid = BoundedVec::try_from([73u8; 10].to_vec()).unwrap();
	let provider_uid = BoundedVec::try_from([74u8; 10].to_vec()).unwrap();
	let entry = RatingInputEntryOf::<Test> {
		entity_uid,
		provider_uid,
		total_encoded_rating: 250u64,
		count_of_txn: 7u64,
		entity_type: EntityTypeOf::Logistic,
		rating_type: RatingTypeOf::Overall,
		provider_did: creator.clone(),
	};
	let entry_digest =
		<Test as frame_system::Config>::Hashing::hash(&[&entry.encode()[..]].concat()[..]);

	let raw_space = [2u8; 256].to_vec();
	let space_digest = <Test as frame_system::Config>::Hashing::hash(&raw_space.encode()[..]);
	let space_id_digest = <Test as frame_system::Config>::Hashing::hash(
		&[&space_digest.encode()[..], &creator.encode()[..]].concat()[..],
	);
	let space_id: SpaceIdOf = generate_space_id::<Test>(&space_id_digest);

	let auth_digest = <Test as frame_system::Config>::Hashing::hash(
		&[&space_id.encode()[..], &creator.encode()[..]].concat()[..],
	);
	let authorization_id: AuthorizationIdOf =
		Ss58Identifier::create_identifier(&auth_digest.encode()[..], IdentifierType::Authorization)
			.unwrap();

	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		// Author Transaction

		assert_ok!(Space::create(
			DoubleOrigin(author.clone(), creator.clone()).into(),
			space_digest,
		));

		assert_ok!(Space::approve(RawOrigin::Root.into(), space_id, 3u64));

		assert_ok!(Score::register_rating(
			DoubleOrigin(author.clone(), creator.clone()).into(),
			entry,
			entry_digest,
			message_id,
			authorization_id,
		));
	});
}
