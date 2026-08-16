//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1099/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1099(t13852: f64, t261: f64, t45: f64, t4731: f64, t3586: f64, t4763: f64, t13798: f64, t13801: f64, t13805: f64, t13807: f64, t13812: f64, t13819: f64, t13823: f64, t13827: f64, t1694: f64, t3001: f64, t3008: f64, t3027: f64, t4735: f64, t4741: f64, t4760: f64, t4765: f64, t960: f64, t972: f64, t9817: f64) -> (f64, f64, f64) {
    let t13854 = 0.62182e-1_f64 * t13852 * t261;
    let t13857 = t45 * t4731;
    let t13860 = t4763 * t3586;
    let t13863 = -t13798 - t13801 + t13805 - 0.34631511798751726598e2_f64 * t960 * t13807 - 0.34631511798751726598e2_f64 * t3001 * t4765 - 0.17315755899375863299e2_f64 * t960 * t13812 + 0.23392893589820816284e1_f64 * t3001 * t4741 - 0.1025389702100779493e4_f64 * t960 * t13819 + 0.23392893589820816284e1_f64 * t960 * t13823 + 0.1038945353962551798e3_f64 * t960 * t13827 - 0.58482233974552040708e0_f64 * t4735 * t3027 + 0.11696446794910408142e1_f64 * t4735 * t3008 - 0.58482233974552040708e0_f64 * t9817 * t1694 - t13854 - 0.11696446794910408142e1_f64 * t3001 * t4760 - 0.11696446794910408142e1_f64 * t13857 * t972 - 0.35089340384731224426e1_f64 * t960 * t13860;
    (t13854, t13860, t13863)
}
