//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 1020/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk1020(t41307: f64, t7603: f64, t36103: f64, t41310: f64, t41313: f64, t25607: f64, t27: f64, t41316: f64, t3851: f64, t39688: f64, t41294: f64, t41298: f64, t41300: f64, t41303: f64, t41305: f64, t41308: f64, t41311: f64, t41315: f64, t41317: f64, t41320: f64, t41321: f64) -> f64 {
    let t41323 = t7603 * t41307;
    let t41324 = 0.33868944250243438616e-2_f64 * t41323;
    let t41325 = t36103 * t41310;
    let t41327 = t7603 * t41313;
    let t41329 = t25607 * t27;
    let t41330 = t41329 * t41316;
    let t41332 = t3851 * t39688;
    let t41334 = 0.84672360625608596544e-3_f64 * t41294 - t41298 - t41300 - t41303 + 0.68186654135613354325e-1_f64 * t41305 + 0.72732431077987577946e-1_f64 * t41308 - 0.2727466165424534173e-1_f64 * t41311 + t41315 - 0.13637330827122670865e0_f64 * t41317 - t41320 + 0.50803416375365157924e-2_f64 * t41321 + t41324 - 0.31752135234603223704e-2_f64 * t41325 + 0.33868944250243438618e-2_f64 * t41327 - 0.7620512456304773689e-2_f64 * t41330 + 0.2993560425465952141e-1_f64 * t41332;
    t41334
}
