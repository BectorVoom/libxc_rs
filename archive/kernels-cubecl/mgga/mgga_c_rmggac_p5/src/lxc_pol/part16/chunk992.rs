//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 992/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk992<F: Float>(t46228: F, t5162: F, t1743: F, t2064: F, t797: F, t46427: F, t5148: F, t9908: F, t46501: F, t5259: F, t40826: F, t9704: F) -> (F, F, F, F, F, F, F) {
    let t46677 = t5162 * t46228;
    let t46685 = t2064 * t1743;
    let t46686 = t797 * t46685;
    let t46702 = t5148 * t46427;
    let t46707 = t9908 * t2064;
    let t46710 = t5259 * t46501;
    let t46715 = t40826 * t9704;
    (t46677, t46685, t46686, t46702, t46707, t46710, t46715)
}
