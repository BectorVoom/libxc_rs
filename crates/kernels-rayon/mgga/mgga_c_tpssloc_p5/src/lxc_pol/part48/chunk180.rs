//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 180/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk180(t109: f64, t107: f64, t626: f64, t106: f64, t38: f64, t606: f64, t95: f64, t103: f64, t100: f64, t92: f64, t96: f64, t64: f64, tau0: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t110 = 1.0_f64 < t109;
    let t654 = t626 * t107 / 3.0_f64;
    let t655 = t106 * t106;
    let t656 = 1.0_f64 / t655;
    let t657 = tau0 * t38;
    let t659 = t606 / 2.0_f64;
    let t660 = t95 * t659;
    let t662 = -t659;
    let t663 = t103 * t662;
    let t666 = 5.0_f64 / 3.0_f64 * t100 * t663 - 5.0_f64 / 3.0_f64 * t657 * t96 + 5.0_f64 / 3.0_f64 * t92 * t660;
    let t667 = t656 * t666;
    let t671 = piecewise3(t110, 0.0_f64, -t654 - t64 * t667 / 8.0_f64);
    (t655, t656, t657, t659, t660, t662, t666, t667, t671)
}
