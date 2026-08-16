//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1278/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1278(t11880: f64, t7111: f64, t11752: f64, t11755: f64, t1017: f64, t11759: f64, t11811: f64, t11824: f64, t25539: f64, t3248: f64, t3255: f64, t7117: f64, t93683: f64, t93685: f64, t93687: f64, t93689: f64, t93691: f64, t93694: f64) -> f64 {
    let t93696 = t7111 * t11880;
    let t93702 = t7111 * t11752;
    let t93704 = t7111 * t11755;
    let t93710 = -0.42874018118069736972e-3_f64 * t7117 * t11811 - 0.17149607247227894789e-2_f64 * t93683 - 0.85748036236139473944e-3_f64 * t93685 - 0.11433071498151929859e-2_f64 * t93687 + 0.17149607247227894789e-2_f64 * t93689 + 11.0_f64 / 108.0_f64 * t93691 * t1017 - t93694 / 54.0_f64 - t93696 / 432.0_f64 - t25539 * t3248 / 36.0_f64 - t25539 * t3255 / 27.0_f64 + t93702 / 288.0_f64 + t93704 / 216.0_f64 + t7111 * t11759 / 288.0_f64 + 7.0_f64 / 648.0_f64 * t7111 * t11824;
    t93710
}
