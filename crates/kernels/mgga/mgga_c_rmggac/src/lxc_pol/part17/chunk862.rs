//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 862/964 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk862<F: Float>(t1707: F, t2084: F, t7599: F, t7603: F, t46164: F, t8764: F, t46167: F, t3826: F, t44732: F, t3851: F, t3839: F, t45720: F, t45726: F, t1614: F, t2350: F, t3810: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t46211 = t2084 * t1707;
    let t46212 = t7599 * t46211;
    let t46214 = t7603 * t46211;
    let t46216 = t8764 * t46164;
    let t46218 = t7599 * t46167;
    let t46220 = t3826 * t44732;
    let t46222 = t3851 * t44732;
    let t46224 = t3839 * t45720;
    let t46226 = t3826 * t45726;
    let t46228 = t2350 * t1614;
    let t46229 = t3810 * t46228;
    (t46212, t46214, t46216, t46218, t46220, t46222, t46224, t46226, t46228, t46229)
}
