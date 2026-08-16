//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 923/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk923(t3061: f64, t8553: f64, t8749: f64, t1102: f64, t1081: f64, t2916: f64, t1094: f64, t3058: f64, t406: f64, t8738: f64, t8697: f64, t8700: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8751 = t8749 * t8553 * t3061;
    let t8753 = 0.1038945353962551798e3_f64 * t1102 * t8751;
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
