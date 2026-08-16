//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 982/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk982(t3826: f64, t45726: f64, t1614: f64, t2350: f64, t3810: f64, t36157: f64, t41324: f64, t41327: f64, t41338: f64, t41341: f64, t41342: f64, t41348: f64, t46212: f64, t46214: f64, t46216: f64, t46218: f64, t46220: f64, t46222: f64, t46224: f64) -> (f64, f64) {
    let t46226 = t3826 * t45726;
    let t46228 = t2350 * t1614;
    let t46229 = t3810 * t46228;
    let t46231 = t41324 + 0.33868944250243438617e-2_f64 * t41327 - 0.15965655602485078086e0_f64 * t41338 - t41341 + 0.31931311204970156171e0_f64 * t41342 + t36157 + 0.72732431077987577947e-1_f64 * t46212 + 0.33868944250243438616e-2_f64 * t46214 + 0.68186654135613354324e-2_f64 * t46216 - 0.13637330827122670865e-1_f64 * t46218 - t41348 - 0.10620923284048465071e-1_f64 * t46220 - 0.15965655602485078085e0_f64 * t46222 - 0.26552308210121162678e-2_f64 * t46224 + 0.39828462315181744016e-2_f64 * t46226 - 0.55759847241254441622e-2_f64 * t46229;
    (t46228, t46231)
}
