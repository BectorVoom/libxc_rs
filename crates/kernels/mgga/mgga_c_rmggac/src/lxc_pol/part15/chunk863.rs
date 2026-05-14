//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 863/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk863<F: Float>(t36157: F, t41324: F, t41327: F, t41338: F, t41341: F, t41342: F, t41348: F, t46212: F, t46214: F, t46216: F, t46218: F, t46220: F, t46222: F, t46224: F, t46226: F, t46229: F) -> (F,) {
    let t46231 = t41324 + 0.33868944250243438617e-2 * t41327 - 0.15965655602485078086e0 * t41338 - t41341 + 0.31931311204970156171e0 * t41342 + t36157 + 0.72732431077987577947e-1 * t46212 + 0.33868944250243438616e-2 * t46214 + 0.68186654135613354324e-2 * t46216 - 0.13637330827122670865e-1 * t46218 - t41348 - 0.10620923284048465071e-1 * t46220 - 0.15965655602485078085e0 * t46222 - 0.26552308210121162678e-2 * t46224 + 0.39828462315181744016e-2 * t46226 - 0.55759847241254441622e-2 * t46229;
    (t46231,)
}
