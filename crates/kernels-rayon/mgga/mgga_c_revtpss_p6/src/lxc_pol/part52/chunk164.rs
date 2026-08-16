//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 164/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk164(t43: f64, t605: f64, t100: f64, t108: f64, t101: f64, t105: f64, t97: f64, tau0: f64) -> (f64, f64, f64, f64, f64) {
    let t656 = tau0 * t43;
    let t658 = t605 / 2.0_f64;
    let t659 = t100 * t658;
    let t661 = -t658;
    let t662 = t108 * t661;
    let t665 = -5.0_f64 / 3.0_f64 * t656 * t101 + 5.0_f64 / 3.0_f64 * t105 * t662 + 5.0_f64 / 3.0_f64 * t97 * t659;
    (t656, t658, t661, t662, t665)
}
