//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 979/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk979<F: Float>(t2115: F, t46129: F, t2118: F, t46177: F, t46181: F, t7633: F, t46185: F, t7641: F, t46116: F, t851: F, t46121: F, t797: F) -> (F, F, F, F, F, F) {
    let t46189 = t2115 * t46129;
    let t46191 = t2118 * t46177;
    let t46193 = t7633 * t46181;
    let t46195 = t7641 * t46185;
    let t46197 = t851 * t46116;
    let t46199 = t797 * t46121;
    (t46189, t46191, t46193, t46195, t46197, t46199)
}
