//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1698/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1698(t3860: f64, t4029: f64, t3857: f64, t4038: f64, t9387: f64, t2608: f64, t3850: f64, t512: f64, t39454: f64, t521: f64, t1333: f64, t9413: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t46279 = t3860 * t4029;
    let t46280 = 144.0_f64 * t46279;
    let t46281 = t3857 * t4029;
    let t46282 = 240.0_f64 * t46281;
    let t46286 = t4038 * t9387;
    let t46287 = 0.23392894490538584828e1_f64 * t46286;
    let t46289 = t512 * t3850 * t2608;
    let t46290 = 6.0_f64 * t46289;
    let t46291 = t39454 * t521;
    let t46292 = 384.0_f64 * t46291;
    let t46297 = 480.0_f64 * t9413 * t1333;
    (t46280, t46282, t46287, t46290, t46292, t46297)
}
