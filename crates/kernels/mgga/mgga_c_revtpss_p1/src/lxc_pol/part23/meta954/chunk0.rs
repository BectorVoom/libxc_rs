//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3173/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3173<F: Float>(t1794: F, t21082: F, t1250: F, t12832: F, t12866: F, t17351: F, t17459: F, t17649: F, t20771: F, t20938: F, t24753: F, t3629: F, t3718: F, t3720: F, t5332: F, t57223: F, t70476: F, t70491: F, t70493: F, t70496: F, t70630: F, t71238: F, t71300: F, t83033: F) -> (F, F) {
    let t83330 = t21082 * t1794;
    let t83352 = -F::cast_from(0.64311027177104605458e-3_f64) * t12832 * t24753 - F::cast_from(0.64311027177104605458e-3_f64) * t3718 * t3720 * t83330 * t1250 + F::cast_from(0.45732285992607719436e-2_f64) * t70476 - t70491 / F::new(288.0) - F::new(11.0) / F::new(324.0) * t70493 + t57223 - F::cast_from(0.17149607247227894789e-2_f64) * t70496 * t20938 + F::cast_from(0.85748036236139473944e-3_f64) * t71238 * t20771 + F::cast_from(0.42874018118069736972e-3_f64) * t17351 * t71300 * t5332 * t3629 - F::cast_from(0.45732285992607719436e-2_f64) * t70630 * t20771 + F::cast_from(0.42874018118069736972e-3_f64) * t12866 * t17649 * t83033 * t17459;
    (t83330, t83352)
}
