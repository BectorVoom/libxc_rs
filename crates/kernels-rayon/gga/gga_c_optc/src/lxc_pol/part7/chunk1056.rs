//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1056/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1056(t669: f64, t6960: f64, t2096: f64, t2105: f64, t664: f64, t6976: f64, t166: f64, t6975: f64, t145: f64, t2107: f64, t2189: f64, t6787: f64, t9896: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t22911 = t6960 * t669;
    let t22915 = t2096 * t2105;
    let t22922 = t664 * t6976;
    let t22932 = 1.0_f64 / t6975 / t166;
    let t22933 = t145 * t22932;
    let t22934 = t2107 * t2107;
    let t22942 = t2189 * t2189;
    let t22949 = t9896 * t6787;
    (t22911, t22915, t22922, t22933, t22934, t22942, t22949)
}
