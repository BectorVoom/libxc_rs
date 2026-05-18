//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 965/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk965<F: Float>(t6400: F, t649: F, t8746: F, t6382: F, t36107: F, t6387: F, t36119: F, t36103: F, t36110: F, t36: F, t5840: F, t262: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t46101 = t649 * t6400;
    let t46102 = t8746 * t46101;
    let t46106 = t649 * t6382;
    let t46107 = t36107 * t46106;
    let t46109 = t649 * t6387;
    let t46110 = t36119 * t46109;
    let t46112 = t36103 * t46106;
    let t46114 = t36110 * t46109;
    let t46116 = t36 * t5840;
    let t46117 = t262 * t46116;
    (t46101, t46102, t46106, t46107, t46109, t46110, t46112, t46114, t46116, t46117)
}
