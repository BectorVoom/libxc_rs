//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 854/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk854<F: Float>(t262: F, t46176: F, t2103: F, t1704: F, t265: F, t7648: F, t1737: F, t7653: F, t2115: F, t46129: F, t2118: F, t7633: F, t7641: F, t46116: F, t851: F, t46121: F, t797: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t46177 = t262 * t46176;
    let t46178 = t2103 * t46177;
    let t46180 = t265 * t1704;
    let t46181 = t262 * t46180;
    let t46182 = t7648 * t46181;
    let t46184 = t265 * t1737;
    let t46185 = t262 * t46184;
    let t46186 = t7653 * t46185;
    let t46189 = t2115 * t46129;
    let t46191 = t2118 * t46177;
    let t46193 = t7633 * t46181;
    let t46195 = t7641 * t46185;
    let t46197 = t851 * t46116;
    let t46199 = t797 * t46121;
    (t46177, t46178, t46180, t46181, t46182, t46184, t46185, t46186, t46189, t46191, t46193, t46195, t46197, t46199)
}
