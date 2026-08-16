//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 977/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk977<F: Float>(t6415: F, t649: F, t8750: F, t6418: F, t7603: F, t46139: F, t46142: F, t8761: F, t1743: F, t265: F, t262: F, t2103: F) -> (F, F, F, F, F, F, F, F, F) {
    let t46164 = t649 * t6415;
    let t46165 = t8750 * t46164;
    let t46167 = t649 * t6418;
    let t46168 = t7603 * t46167;
    let t46170 = t7603 * t46139;
    let t46172 = t8761 * t46142;
    let t46176 = t265 * t1743;
    let t46177 = t262 * t46176;
    let t46178 = t2103 * t46177;
    (t46164, t46165, t46167, t46168, t46170, t46172, t46176, t46177, t46178)
}
