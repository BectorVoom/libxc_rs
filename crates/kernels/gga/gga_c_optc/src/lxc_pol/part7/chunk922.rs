//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 922/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk922<F: Float>(t1085: F, t1094: F, t8738: F, t1102: F, t241: F, t3029: F, t1104: F, t2919: F, t3067: F, t3057: F, t411: F) -> (F, F, F, F, F, F) {
    let t8740 = t1085 * t8738 * t1094;
    let t8742 = F::cast_from(0.58482233974552040708e0_f64) * t1102 * t8740;
    let t8743 = t241 * t3029;
    let t8745 = F::cast_from(0.17544670192365612213e1_f64) * t8743 * t1104;
    let t8747 = F::cast_from(0.35089340384731224426e1_f64) * t3067 * t2919;
    let t8749 = F::new(1.0) / t3057 / t411;
    (t8740, t8742, t8743, t8745, t8747, t8749)
}
