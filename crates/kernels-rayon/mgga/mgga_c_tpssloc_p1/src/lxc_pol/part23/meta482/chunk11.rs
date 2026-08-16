//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1462/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1462(t6052: f64, t11310: f64, t11350: f64, t1137: f64, t11420: f64, t15136: f64, t15146: f64, t1682: f64, t18650: f64, t21836: f64, t21907: f64, t21952: f64, t3332: f64, t3357: f64, t3359: f64, t3403: f64, t436: f64, t51680: f64, t6037: f64, t6069: f64, t63454: f64, t71729: f64, t78287: f64, t78359: f64, t78361: f64, t78364: f64, t78367: f64, t78370: f64, t78373: f64, t78859: f64, t78961: f64, t78973: f64) -> f64 {
    let t78988 = t6052 * t6052;
    let t79002 = 36.0_f64 * t3357 * t6037 * t6052 - 0.14035736694323150897e2_f64 * t15136 * t21836 - 0.310907e-1_f64 * (t78961 + t78973) * t436 + t78359 - t78361 + t78364 + t78367 - t78370 - t78373 + 0.12865583598954028054e3_f64 * t3357 * t71729 * t1682 + 0.12414243100625616072e5_f64 * t11350 * t18650 * t6052 + 24.0_f64 * t15146 * t21952 - 24.0_f64 * t11420 * t78859 * t1137 - 6.0_f64 * t3332 * t78988 * t1137 + 0.96491876992155210402e2_f64 * t3357 * t78988 * t3359 - 0.70178683471615754484e1_f64 * t63454 * t6069 - 0.4155806185363551302e3_f64 * t51680 * t21907 + 0.6233709278045326953e3_f64 * t11310 * t78287 * t3403;
    t79002
}
