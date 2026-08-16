//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3173/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3173(t1794: f64, t21082: f64, t1250: f64, t12832: f64, t12866: f64, t17351: f64, t17459: f64, t17649: f64, t20771: f64, t20938: f64, t24753: f64, t3629: f64, t3718: f64, t3720: f64, t5332: f64, t57223: f64, t70476: f64, t70491: f64, t70493: f64, t70496: f64, t70630: f64, t71238: f64, t71300: f64, t83033: f64) -> (f64, f64) {
    let t83330 = t21082 * t1794;
    let t83352 = -0.64311027177104605458e-3_f64 * t12832 * t24753 - 0.64311027177104605458e-3_f64 * t3718 * t3720 * t83330 * t1250 + 0.45732285992607719436e-2_f64 * t70476 - t70491 / 288.0_f64 - 11.0_f64 / 324.0_f64 * t70493 + t57223 - 0.17149607247227894789e-2_f64 * t70496 * t20938 + 0.85748036236139473944e-3_f64 * t71238 * t20771 + 0.42874018118069736972e-3_f64 * t17351 * t71300 * t5332 * t3629 - 0.45732285992607719436e-2_f64 * t70630 * t20771 + 0.42874018118069736972e-3_f64 * t12866 * t17649 * t83033 * t17459;
    (t83330, t83352)
}
