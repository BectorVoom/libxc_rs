//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 982/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk982<F: Float>(t3826: F, t45726: F, t1614: F, t2350: F, t3810: F, t36157: F, t41324: F, t41327: F, t41338: F, t41341: F, t41342: F, t41348: F, t46212: F, t46214: F, t46216: F, t46218: F, t46220: F, t46222: F, t46224: F) -> (F, F) {
    let t46226 = t3826 * t45726;
    let t46228 = t2350 * t1614;
    let t46229 = t3810 * t46228;
    let t46231 = t41324 + F::new(0.33868944250243438617e-2) * t41327 - F::new(0.15965655602485078086e0) * t41338 - t41341 + F::new(0.31931311204970156171e0) * t41342 + t36157 + F::new(0.72732431077987577947e-1) * t46212 + F::new(0.33868944250243438616e-2) * t46214 + F::new(0.68186654135613354324e-2) * t46216 - F::new(0.13637330827122670865e-1) * t46218 - t41348 - F::new(0.10620923284048465071e-1) * t46220 - F::new(0.15965655602485078085e0) * t46222 - F::new(0.26552308210121162678e-2) * t46224 + F::new(0.39828462315181744016e-2) * t46226 - F::new(0.55759847241254441622e-2) * t46229;
    (t46228, t46231)
}
