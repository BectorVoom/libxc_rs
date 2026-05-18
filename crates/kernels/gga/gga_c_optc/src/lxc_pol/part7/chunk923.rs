//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 923/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk923<F: Float>(t3061: F, t8553: F, t8749: F, t1102: F, t1081: F, t2916: F, t1094: F, t3058: F, t406: F, t8738: F, t8697: F, t8700: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t8751 = t8749 * t8553 * t3061;
    let t8753 = F::new(0.1038945353962551798e3) * t1102 * t8751;
    let t8754 = t1081 * t2916;
    let t8757 = t8553 * t1094;
    let t8762 = t1081 * t3058;
    let t8765 = t406 * t8749;
    let t8766 = t8553 * t3061;
    let t8769 = t8738 * t1094;
    let t8772 = t406 * t8697;
    let t8773 = t8553 * t8700;
    (t8751, t8753, t8754, t8757, t8762, t8765, t8766, t8769, t8772, t8773)
}
