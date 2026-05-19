//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 342/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk342<F: Float>(t1085: F, t1093: F, t1094: F, t1102: F, t23: F, t429: F, t116: F, t428: F, t427: F, t861: F) -> (F, F, F, F, F) {
    let t1104 = t1085 * t1093 * t1094;
    let t1106 = F::cast_from(0.58482233974552040708e0_f64) * t1102 * t1104;
    let t1107 = t23 * t429;
    let t1108 = t116 * t1107;
    let t1110 = t428 * t1108 / F::new(288.0);
    let t1111 = t427 * t861;
    (t1104, t1106, t1107, t1110, t1111)
}
