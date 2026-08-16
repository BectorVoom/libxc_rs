//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1212/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1212(t94696: f64, t94698: f64, t7282: f64, t93139: f64, t2453: f64, t26053: f64, t7289: f64, t94600: f64, t2028: f64, t3999: f64, t25875: f64, t25894: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t94700 = 0.43639970290213137151e-3_f64 * t94696 * t94698;
    let t94701 = t93139 * t7282;
    let t94703 = 0.51727911450665971904e-3_f64 * t94701 * t94698;
    let t94725 = t2453 * t26053;
    let t94761 = 0.39982213492741449076e-1_f64 * t7289 * t94600;
    let t94762 = t2028 * t3999;
    let t94763 = t25875 * t94762;
    let t94768 = t25894 * t94762;
    (t94700, t94703, t94725, t94761, t94763, t94768)
}
