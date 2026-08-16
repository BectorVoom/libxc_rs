//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1739/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1739(t46979: f64, t1320: f64, t9561: f64, t9554: f64, t39483: f64, t39520: f64, t39528: f64, t39531: f64, t46968: f64, t46970: f64, t46972: f64, t46974: f64, t46976: f64, t46978: f64) -> (f64, f64, f64, f64) {
    let t46980 = 192.0_f64 * t46979;
    let t46981 = t1320 * t9561;
    let t46982 = 16.0_f64 * t46981;
    let t46983 = t1320 * t9554;
    let t46984 = 48.0_f64 * t46983;
    let t46985 = t46968 + t46970 + t46972 - t39483 + t39520 - t39528 - t46974 + t39531 + t46976 - t46978 - t46980 - t46982 - t46984;
    (t46980, t46982, t46984, t46985)
}
