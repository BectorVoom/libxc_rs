//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1099/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1099<F: Float>(t13852: F, t261: F, t45: F, t4731: F, t3586: F, t4763: F, t13798: F, t13801: F, t13805: F, t13807: F, t13812: F, t13819: F, t13823: F, t13827: F, t1694: F, t3001: F, t3008: F, t3027: F, t4735: F, t4741: F, t4760: F, t4765: F, t960: F, t972: F, t9817: F) -> (F, F, F) {
    let t13854 = F::cast_from(0.62182e-1_f64) * t13852 * t261;
    let t13857 = t45 * t4731;
    let t13860 = t4763 * t3586;
    let t13863 = -t13798 - t13801 + t13805 - F::cast_from(0.34631511798751726598e2_f64) * t960 * t13807 - F::cast_from(0.34631511798751726598e2_f64) * t3001 * t4765 - F::cast_from(0.17315755899375863299e2_f64) * t960 * t13812 + F::cast_from(0.23392893589820816284e1_f64) * t3001 * t4741 - F::cast_from(0.1025389702100779493e4_f64) * t960 * t13819 + F::cast_from(0.23392893589820816284e1_f64) * t960 * t13823 + F::cast_from(0.1038945353962551798e3_f64) * t960 * t13827 - F::cast_from(0.58482233974552040708e0_f64) * t4735 * t3027 + F::cast_from(0.11696446794910408142e1_f64) * t4735 * t3008 - F::cast_from(0.58482233974552040708e0_f64) * t9817 * t1694 - t13854 - F::cast_from(0.11696446794910408142e1_f64) * t3001 * t4760 - F::cast_from(0.11696446794910408142e1_f64) * t13857 * t972 - F::cast_from(0.35089340384731224426e1_f64) * t960 * t13860;
    (t13854, t13860, t13863)
}
