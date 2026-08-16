//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2126/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2126(t6049: f64, t689: f64, t7014: f64, t106128: f64, t25375: f64, t18805: f64, t93261: f64, t231: f64, t25383: f64, t25392: f64, t27189: f64, t27353: f64, t27357: f64, t29675: f64, t4423: f64, t4534: f64, t6016: f64, t62604: f64, t62695: f64, t7048: f64, t7070: f64, t7076: f64, t7759: f64, t93276: f64, t93278: f64, t99344: f64, t99346: f64, t99351: f64) -> f64 {
    let t106316 = t689 * t7014 * t6049;
    let t106318 = t25375 * t106128;
    let t106326 = t93261 * t18805;
    let t106342 = -0.10975748638225852664e-1_f64 * t106316 - 0.28912093960683998207e-1_f64 * t106318 - 0.8673628188205199462e0_f64 * t27353 * t27357 * t62604 + 0.4336814094102599731e0_f64 * t27353 * t25392 * t62695 + 0.19514881078765566037e-1_f64 * t106326 + 0.8673628188205199462e0_f64 * t7070 * t7076 * t7759 * t4423 * t231 + 0.4336814094102599731e0_f64 * t25383 * t29675 + 0.4336814094102599731e0_f64 * t7070 * t7076 * t7048 * t6016 * t231 - t93276 - t99344 + t99346 - 0.13170898365871023197e1_f64 * t27189 * t4534 + t93278 + t99351;
    t106342
}
