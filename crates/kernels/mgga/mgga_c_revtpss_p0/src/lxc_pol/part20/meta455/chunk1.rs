//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1739/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1739<F: Float>(t46979: F, t1320: F, t9561: F, t9554: F, t39483: F, t39520: F, t39528: F, t39531: F, t46968: F, t46970: F, t46972: F, t46974: F, t46976: F, t46978: F) -> (F, F, F, F) {
    let t46980 = F::new(192.0) * t46979;
    let t46981 = t1320 * t9561;
    let t46982 = F::new(16.0) * t46981;
    let t46983 = t1320 * t9554;
    let t46984 = F::new(48.0) * t46983;
    let t46985 = t46968 + t46970 + t46972 - t39483 + t39520 - t39528 - t46974 + t39531 + t46976 - t46978 - t46980 - t46982 - t46984;
    (t46980, t46982, t46984, t46985)
}
