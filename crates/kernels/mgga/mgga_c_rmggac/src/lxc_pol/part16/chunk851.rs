//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 851/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk851<F: Float>(t6387: F, t649: F, t36119: F, t36103: F, t46106: F, t36110: F, t36: F, t5840: F, t262: F, t2115: F, t6376: F, t2118: F, t22: F, t30526: F, t9885: F, t1734: F, t265: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t46109 = t649 * t6387;
    let t46110 = t36119 * t46109;
    let t46112 = t36103 * t46106;
    let t46114 = t36110 * t46109;
    let t46116 = t36 * t5840;
    let t46117 = t262 * t46116;
    let t46118 = t2115 * t46117;
    let t46121 = t36 * t6376;
    let t46122 = t262 * t46121;
    let t46123 = t2118 * t46122;
    let t46126 = t30526 * t22 * t9885;
    let t46128 = t265 * t1734;
    (t46109, t46110, t46112, t46114, t46116, t46117, t46118, t46121, t46122, t46123, t46126, t46128)
}
