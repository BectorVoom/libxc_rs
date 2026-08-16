//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 894/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk894(t16708: f64, t818: f64, t16861: f64, t799: f64, t2416: f64, t16784: f64, t837: f64, t10409: f64, t10419: f64, t10485: f64, t16785: f64, t16817: f64, t16820: f64, t16824: f64, t16826: f64, t16828: f64, t16860: f64, t16864: f64, t16866: f64, t16869: f64, t2518: f64, t2537: f64, t3754: f64, t4869: f64, t4904: f64, t4920: f64, t4923: f64, t7753: f64, t7813: f64, t829: f64) -> (f64, f64, f64, f64, f64) {
    let t16872 = t16708 * t818;
    let t16875 = t16861 * t799;
    let t16877 = 6.0_f64 * t2416 * t16875;
    let t16880 = t16784 * t837;
    let t16883 = 0.17544670192365612213e1_f64 * t3754 * t4920 + 0.51947267698127589899e2_f64 * t10409 * t4923 - 0.1038945353962551798e3_f64 * t7813 * t16785 + 0.58482233974552040708e0_f64 * t829 * t16817 + 0.1025389702100779493e4_f64 * t7753 * t16820 - t16824 - t16826 - t16828 - t16860 - t16864 + t16866 + t16869 - 6.0_f64 * t10419 * t4869 + 6.0_f64 * t2518 * t16872 - t16877 - 0.35089340384731224426e1_f64 * t10485 * t4904 + 0.35089340384731224426e1_f64 * t2537 * t16880;
    (t16872, t16875, t16877, t16880, t16883)
}
