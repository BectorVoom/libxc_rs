//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2446/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2446(t40082: f64, t512: f64, t520: f64, t1333: f64, t9410: f64, t1320: f64, t9428: f64, t1331: f64, t9413: f64, t3853: f64, t3863: f64, t1340: f64, t40086: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t46970 = t512 * t520 * t40082;
    let t46971 = t9410 * t1333;
    let t46973 = t1320 * t9428;
    let t46975 = t9410 * t1331;
    let t46977 = t9413 * t1331;
    let t46979 = t3863 * t3853;
    let t46988 = 0.62337092780453269531e3_f64 * t1340 * t40086;
    (t46970, t46971, t46973, t46975, t46977, t46979, t46988)
}
