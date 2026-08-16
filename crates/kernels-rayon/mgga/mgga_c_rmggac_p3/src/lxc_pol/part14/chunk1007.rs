//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 1007/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk1007(t25636: f64, t40901: f64, t2347: f64, t25525: f64, t794: f64, t3839: f64, t40905: f64, t25518: f64, t38564: f64, t41132: f64, t41134: f64, t41136: f64, t41138: f64, t41140: f64, t41142: f64, t41144: f64, t41146: f64, t41148: f64, t41151: f64, t41153: f64) -> f64 {
    let t41155 = t25636 * t40901;
    let t41158 = t25525 * t2347 * t794;
    let t41160 = t3839 * t40905;
    let t41162 = t25518 * t38564;
    let t41164 = 0.1814407727691612783e-2_f64 * t41132 + 0.5987120850931904282e-1_f64 * t41134 + 0.5987120850931904282e-1_f64 * t41136 + 0.5987120850931904282e-1_f64 * t41138 + 0.2993560425465952141e-1_f64 * t41140 - 0.13276154105060581339e-2_f64 * t41142 - 0.5987120850931904282e-1_f64 * t41144 - 0.15965655602485078085e0_f64 * t41146 + 0.2993560425465952141e0_f64 * t41148 - 0.5454932330849068346e-1_f64 * t41151 + 0.13637330827122670865e0_f64 * t41153 + 0.22303938896501776649e-1_f64 * t41155 - 0.39828462315181744017e-2_f64 * t41158 + 0.70806155226989767141e-2_f64 * t41160 - 0.13939961810313610406e-1_f64 * t41162;
    t41164
}
