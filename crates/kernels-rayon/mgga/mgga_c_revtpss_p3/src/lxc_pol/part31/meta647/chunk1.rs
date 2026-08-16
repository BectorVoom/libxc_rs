//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2125/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2125(t6072: f64, t689: f64, t7014: f64, t5978: f64, t886: f64, t1558: f64, t231: f64, t4533: f64, t25391: f64, t25392: f64, t27199: f64, t27292: f64, t27313: f64, t27350: f64, t27353: f64, t62624: f64, t62637: f64, t93252: f64, t93272: f64, t93273: f64, t99191: f64, t99307: f64, t99313: f64, t99323: f64, t99342: f64) -> f64 {
    let t106286 = t689 * t7014 * t6072;
    let t106290 = t5978 * t886;
    let t106302 = t4533 * t1558 * t231;
    let t106313 = 0.54878743191129263322e-2_f64 * t106286 - 0.17347256376410398924e1_f64 * t99191 * t27313 - 0.8673628188205199462e0_f64 * t25391 * t25392 * t106290 + 0.8673628188205199462e0_f64 * t27353 * t25392 * t62624 + 0.11565819519348392139e-2_f64 * t93252 - 0.17347256376410398924e1_f64 * t99191 * t27350 - 0.26019841438354088051e-1_f64 * t99307 - 0.17347256376410398924e1_f64 * t25391 * t25392 * t106302 + 0.4336814094102599731e0_f64 * t27353 * t25392 * t62637 - 0.23131639038696784278e-2_f64 * t99313 + t99323 + 0.8673628188205199462e0_f64 * t27199 * t27292 + t93272 + 0.13009920719177044025e-1_f64 * t93273 + t99342;
    t106313
}
