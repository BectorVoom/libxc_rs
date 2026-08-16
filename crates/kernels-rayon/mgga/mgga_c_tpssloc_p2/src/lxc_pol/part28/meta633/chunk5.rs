//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 2006/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2006(t27143: f64, t532: f64, t90459: f64, t90468: f64, t90470: f64, t90472: f64, t225: f64, t27137: f64, t27059: f64, t2091: f64, t40590: f64, t1386: f64, t16474: f64, t24082: f64, t26224: f64, t5354: f64, t80647: f64, t80659: f64, t80663: f64, t80665: f64, t80667: f64, t80671: f64, t90462: f64, t90466: f64, t90477: f64, t90485: f64, t90491: f64, t90498: f64) -> (f64, f64) {
    let t93286 = t532 * t27143;
    let t93306 = 0.76763589786250567036e-1_f64 * t90459;
    let t93309 = 0.15352717957250113407e0_f64 * t90468;
    let t93310 = 0.15352717957250113407e0_f64 * t90470;
    let t93311 = 0.15352717957250113407e0_f64 * t90472;
    let t93313 = t27137 * t225;
    let t93316 = t27059 * t225;
    let t93319 = t40590 * t2091;
    let t93332 = -2.0_f64 * t24082 * t5354 + t93306 + 0.6579736267392905746e-1_f64 * t90462 + 0.3289868133696452873e-1_f64 * t90466 + t93309 + t93310 - t93311 + 0.6579736267392905746e-1_f64 * t90477 - 2.0_f64 * t93313 * t1386 - 2.0_f64 * t93316 * t1386 + 24.0_f64 * t26224 * t93319 * t16474 + 0.16449340668482264365e-1_f64 * t80647 - 0.9869604401089358619e-1_f64 * t90485 + 0.16449340668482264365e-1_f64 * t80659 - 0.6579736267392905746e-1_f64 * t90491 - 0.25587863262083522346e0_f64 * t80663 + 0.15352717957250113407e0_f64 * t80665 + 0.76763589786250567036e-1_f64 * t80667 - 0.20835831513410868196e0_f64 * t80671 - 0.46058153871750340222e0_f64 * t90498;
    (t93286, t93332)
}
