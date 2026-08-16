//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2165/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2165(t2453: f64, t27212: f64, t25301: f64, t25410: f64, t7774: f64, t93240: f64, t14662: f64, t1949: f64, t231: f64, t27350: f64, t27354: f64, t4423: f64, t7048: f64, t7070: f64, t7076: f64, t92917: f64, t93180: f64, t93184: f64, t93192: f64, t93195: f64, t99228: f64, t99231: f64, t99234: f64, t99237: f64, t99243: f64, t99245: f64) -> f64 {
    let t99257 = t2453 * t27212;
    let t99258 = t99257 * t25301;
    let t99261 = t93240 * t25410 * t7774;
    let t99264 = 0.39029762157531132075e-1_f64 * t99228 + t99231 + 0.72280234901709995518e-2_f64 * t93180 + t99234 - 0.17347256376410398924e1_f64 * t92917 * t27350 + 0.8673628188205199462e0_f64 * t99237 * t27354 + 0.19274729307122665471e-1_f64 * t93184 - t99243 + t99245 + 0.91399340044406952588e-2_f64 * t93192 + 0.8673628188205199462e0_f64 * t7070 * t7076 * t7048 * t4423 * t231 + 0.4336814094102599731e0_f64 * t7070 * t7076 * t1949 * t14662 * t231 + 0.17135234354032049604e-2_f64 * t99258 + 0.24093411633903331839e-3_f64 * t99261 - 0.23131639038696784278e-2_f64 * t93195;
    t99264
}
