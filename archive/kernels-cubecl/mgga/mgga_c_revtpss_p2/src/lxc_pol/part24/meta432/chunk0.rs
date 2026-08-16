//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1382/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1382<F: Float>(t46072: F, t59: F, t2681: F, t64: F, t112: F, t10207: F, t111: F, t36227: F, t36415: F, t39454: F, t521: F, t1333: F, t9413: F) -> (F, F, F, F, F, F, F, F) {
    let t46074 = F::cast_from(1.0_f64) / t59 / t46072;
    let t46089 = t64 * t2681;
    let t46090 = F::cast_from(20944.0_f64) / F::cast_from(81.0_f64) * t46089;
    let t46143 = F::cast_from(2618.0_f64) / F::cast_from(81.0_f64) * t46089 * t112;
    let t46157 = F::cast_from(1.0_f64) / t10207 / t111;
    let t46196 = F::cast_from(1.0_f64) / t36227;
    let t46212 = F::cast_from(1.0_f64) / t36415;
    let t46291 = t39454 * t521;
    let t46292 = F::cast_from(384.0_f64) * t46291;
    let t46297 = F::cast_from(480.0_f64) * t9413 * t1333;
    (t46074, t46090, t46143, t46157, t46196, t46212, t46292, t46297)
}
