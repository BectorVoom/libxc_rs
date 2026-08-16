//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1402/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1402(t1199: f64, t9292: f64, t2879: f64, t2885: f64, t1196: f64, t9303: f64, t481: f64, t484: f64, t9302: f64, t2887: f64, t2900: f64, t26261: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t27918 = t9292 * t1199;
    let t27921 = t2879 * t2885;
    let t27926 = t1196 * t9303;
    let t27935 = t481 / t9302 / t484;
    let t27936 = t2887 * t2887;
    let t27942 = t2900 * t2900;
    let t27950 = 0.75383950617283950617e4_f64 * t26261;
    (t27918, t27921, t27926, t27935, t27936, t27942, t27950)
}
