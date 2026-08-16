//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1667/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1667(t88166: f64, t88218: f64, t88242: f64, t88262: f64, t41672: f64, t77499: f64, t77505: f64, t77507: f64, t77509: f64, t77663: f64, t77667: f64, t88089: f64, t88097: f64, t88144: f64, t88147: f64, t88150: f64, t88161: f64, t88164: f64) -> (f64, f64) {
    let t88264 = t88166 + t88218 + t88242 + t88262;
    let t88291 = -0.10805407407407407407e0_f64 * t88144 - 0.104195e0_f64 * t88147 + 0.55570666666666666666e0_f64 * t88150 - 0.55570666666666666668e0_f64 * t77663 + 0.12349037037037037037e0_f64 * t77667 - 0.185931e2_f64 * t88089 + 0.41318e1_f64 * t88097 + t41672 + 0.76514814814814814814e0_f64 * t77499 + 0.68863333333333333332e0_f64 * t77505 - 0.27545333333333333332e1_f64 * t77507 + 0.41318e1_f64 * t77509 - 0.125034e1_f64 * t88161 - 0.104195e0_f64 * t88164;
    (t88264, t88291)
}
