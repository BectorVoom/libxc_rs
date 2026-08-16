//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1382/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1382(t46072: f64, t59: f64, t2681: f64, t64: f64, t112: f64, t10207: f64, t111: f64, t36227: f64, t36415: f64, t39454: f64, t521: f64, t1333: f64, t9413: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t46074 = 1.0_f64 / t59 / t46072;
    let t46089 = t64 * t2681;
    let t46090 = 20944.0_f64 / 81.0_f64 * t46089;
    let t46143 = 2618.0_f64 / 81.0_f64 * t46089 * t112;
    let t46157 = 1.0_f64 / t10207 / t111;
    let t46196 = 1.0_f64 / t36227;
    let t46212 = 1.0_f64 / t36415;
    let t46291 = t39454 * t521;
    let t46292 = 384.0_f64 * t46291;
    let t46297 = 480.0_f64 * t9413 * t1333;
    (t46074, t46090, t46143, t46157, t46196, t46212, t46292, t46297)
}
