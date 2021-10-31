use primitive_types::U256;

#[test]
fn u256_iter() {
	let mut sum = 0u64;
	for x in U256::zero()..=(U256::one() * 10) {
		sum += x.as_u64();
	}
	assert_eq!(sum, 55u64);
}
