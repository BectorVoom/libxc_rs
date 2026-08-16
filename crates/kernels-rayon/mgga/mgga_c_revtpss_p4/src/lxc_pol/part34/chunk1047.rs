//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1047/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1047(t12542: f64, t12543: f64, t24238: f64, t24242: f64, t24246: f64, t24250: f64, t24289: f64, t24292: f64, t24295: f64, t24298: f64, t24313: f64, t24315: f64, t24318: f64, t24320: f64) -> f64 {
    let t24406 = 0.181155e1_f64 * t24242 + 0.301925e0_f64 * t24250 - 0.16557e0_f64 * t24289 + 0.49671e0_f64 * t24292 + 0.82785e-1_f64 * t24295 - t12542 - t12543 - 0.82785e-1_f64 * t24298 - 0.60384999999999999999e0_f64 * t24238 + 0.181155e1_f64 * t24246 + 0.16504875e0_f64 * t24313 + 0.258925e1_f64 * t24315 + 0.19419375e1_f64 * t24318 - 0.412621875e-1_f64 * t24320;
    t24406
}
