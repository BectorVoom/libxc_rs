//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2197/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2197(t1398: f64, t14224: f64, t25930: f64, t30055: f64, t543: f64, t7295: f64, t7301: f64, t94677: f64, t94682: f64, t97869: f64, t97882: f64, t97894: f64, t97900: f64, t97908: f64, t97915: f64, t97917: f64, t97920: f64, t97923: f64, t97926: f64, t98340: f64) -> f64 {
    let t108327 = 0.17135234354032049604e-1_f64 * t94677 + t97869 - 0.23131639038696784278e-2_f64 * t97882 - 0.17347256376410398924e1_f64 * t25930 * t98340 * t14224 - 0.13009920719177044025e-2_f64 * t97894 + 0.19274729307122665472e-1_f64 * t97900 + t94682 - t97908 + t97915 + 0.3427046870806409921e-2_f64 * t97917 + 0.4336814094102599731e0_f64 * t7295 * t7301 * t30055 * t1398 * t543 - t97920 + 0.3427046870806409921e-2_f64 * t97923 - 0.19274729307122665472e-1_f64 * t97926;
    t108327
}
