//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1392/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1392(t4086: f64, t9801: f64, t1320: f64, t9545: f64, t40082: f64, t512: f64, t520: f64, t1333: f64, t9410: f64, t3853: f64, t3863: f64, t1340: f64, t40086: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t46946 = t9801 * t4086;
    let t46963 = 16.0_f64 * t1320 * t9545;
    let t46970 = t512 * t520 * t40082;
    let t46971 = t9410 * t1333;
    let t46972 = 960.0_f64 * t46971;
    let t46979 = t3863 * t3853;
    let t46980 = 192.0_f64 * t46979;
    let t46988 = 0.62337092780453269531e3_f64 * t1340 * t40086;
    (t46946, t46963, t46970, t46972, t46980, t46988)
}
