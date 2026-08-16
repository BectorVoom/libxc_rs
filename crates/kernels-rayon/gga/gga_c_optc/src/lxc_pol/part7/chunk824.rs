//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 824/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk824(t2488: f64, t2513: f64, t2521: f64, t2525: f64, t2531: f64, t2534: f64, t2538: f64, t7675: f64, t7678: f64, t7684: f64, t7691: f64, t7753: f64, t7754: f64, t7759: f64, t7760: f64, t7794: f64, t7799: f64, t7802: f64, t7805: f64, t7810: f64, t7813: f64, t7814: f64, t7817: f64, t7820: f64, t7825: f64, t7828: f64, t810: f64, t819: f64, t829: f64, t838: f64) -> f64 {
    let t7831 = -t7675 + t7678 + t7684 - t7691 + 0.1025389702100779493e4_f64 * t7753 * t7754 - 0.19298809906722418785e3_f64 * t7759 * t7760 + 1.0_f64 * t810 * t7794 + 0.20691336878655965246e4_f64 * t7799 * t7802 + 0.17544670192365612213e1_f64 * t7805 * t838 + 0.17544670192365612213e1_f64 * t2525 * t2534 + 0.51947267698127589899e2_f64 * t7810 * t2538 - 0.1038945353962551798e3_f64 * t7813 * t7814 + 0.58482233974552040708e0_f64 * t829 * t7817 + 3.0_f64 * t7820 * t819 + 3.0_f64 * t2488 * t2513 + 0.96494049533612093922e2_f64 * t7825 * t2521 - 0.35089340384731224426e1_f64 * t7828 * t2531;
    t7831
}
