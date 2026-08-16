//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2173/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2173(t689: f64, t6896: f64, t7242: f64, t22399: f64, t26054: f64, t108282: f64, t25930: f64, t27837: f64, t27841: f64, t27972: f64, t543: f64, t6843: f64, t7274: f64, t7295: f64, t7298: f64, t7301: f64, t7921: f64, t94784: f64, t94807: f64, t94820: f64, t94842: f64, t97875: f64, t98010: f64, t98011: f64, t98029: f64, t98050: f64) -> f64 {
    let t108411 = t689 * t7242 * t6896;
    let t108422 = t26054 * t22399;
    let t108425 = 0.17347256376410398924e1_f64 * t98050 * t7921 + t94784 - 0.17347256376410398924e1_f64 * t25930 * t97875 * t27972 - t98010 + 0.34270468708064099208e-1_f64 * t98011 - 0.52041769129231196772e1_f64 * t27837 * t27841 + 0.17135234354032049604e-2_f64 * t94807 - 0.10975748638225852664e-1_f64 * t108411 + 0.4336814094102599731e0_f64 * t7295 * t7301 * t7274 * t6843 * t543 - 0.24093411633903331839e-3_f64 * t94820 + 0.38549458614245330944e-1_f64 * t98029 + 0.8673628188205199462e0_f64 * t108282 * t7298 - 0.9757440539382783019e-2_f64 * t108422 + 0.96373646535613327357e-2_f64 * t94842;
    t108425
}
