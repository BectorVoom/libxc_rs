//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 972/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk972<F: Float>(t46167: F, t7599: F, t3826: F, t44732: F, t3851: F, t3839: F, t45720: F, t45726: F, t1614: F, t2350: F, t3810: F, t30526: F, t9708: F) -> (F, F, F, F, F, F, F, F) {
    let t46218 = t7599 * t46167;
    let t46220 = t3826 * t44732;
    let t46222 = t3851 * t44732;
    let t46224 = t3839 * t45720;
    let t46226 = t3826 * t45726;
    let t46228 = t2350 * t1614;
    let t46229 = t3810 * t46228;
    let t46232 = t30526 * t9708;
    (t46218, t46220, t46222, t46224, t46226, t46228, t46229, t46232)
}
