//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2557/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2557(t4086: f64, t9801: f64, t9846: f64, t9744: f64, t9966: f64, t3855: f64, t3860: f64, t1320: f64, t9545: f64, t3857: f64, t40082: f64, t512: f64, t520: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t46946 = t9801 * t4086;
    let t46947 = t46946 * t9846;
    let t46949 = t9744 * t9966;
    let t46960 = t3860 * t3855;
    let t46963 = 16.0_f64 * t1320 * t9545;
    let t46967 = t3857 * t3855;
    let t46970 = t512 * t520 * t40082;
    (t46946, t46947, t46949, t46960, t46963, t46967, t46970)
}
