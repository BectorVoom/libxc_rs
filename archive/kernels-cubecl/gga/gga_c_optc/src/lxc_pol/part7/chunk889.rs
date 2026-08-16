//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 889/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk889<F: Float>(t1094: F, t3058: F, t8553: F, t1102: F, t1032: F, t2992: F, t2995: F, t1055: F, t2994: F, t1056: F, t3018: F, t1057: F, t3012: F) -> (F, F, F, F, F, F, F, F) {
    let t8555 = t3058 * t8553 * t1094;
    let t8557 = F::cast_from(0.35089340384731224426e1_f64) * t1102 * t8555;
    let t8558 = t1032 * t2992;
    let t8560 = F::cast_from(6.0_f64) * t8558 * t2995;
    let t8561 = t2994 * t1055;
    let t8562 = t8561 * t1056;
    let t8564 = F::cast_from(6.0_f64) * t3018 * t8562;
    let t8565 = t1057 * t3012;
    (t8555, t8557, t8558, t8560, t8561, t8562, t8564, t8565)
}
