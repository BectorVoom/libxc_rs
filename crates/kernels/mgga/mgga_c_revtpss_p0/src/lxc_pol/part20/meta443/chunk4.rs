//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1698/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1698<F: Float>(t3860: F, t4029: F, t3857: F, t4038: F, t9387: F, t2608: F, t3850: F, t512: F, t39454: F, t521: F, t1333: F, t9413: F) -> (F, F, F, F, F, F) {
    let t46279 = t3860 * t4029;
    let t46280 = F::new(144.0) * t46279;
    let t46281 = t3857 * t4029;
    let t46282 = F::new(240.0) * t46281;
    let t46286 = t4038 * t9387;
    let t46287 = F::cast_from(0.23392894490538584828e1_f64) * t46286;
    let t46289 = t512 * t3850 * t2608;
    let t46290 = F::new(6.0) * t46289;
    let t46291 = t39454 * t521;
    let t46292 = F::new(384.0) * t46291;
    let t46297 = F::new(480.0) * t9413 * t1333;
    (t46280, t46282, t46287, t46290, t46292, t46297)
}
