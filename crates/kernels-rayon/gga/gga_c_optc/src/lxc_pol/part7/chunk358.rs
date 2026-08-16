//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 358/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk358(t474: f64, t1023: f64, t1049: f64, t1030: f64, t1053: f64, t484: f64, t481: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1188 = 1.0_f64 / t474;
    let t1192 = 0.19388333333333333333e1_f64 * t1023;
    let t1194 = 0.12315e-2_f64 * t1049;
    let t1196 = -t1192 - 0.19388333333333333333e1_f64 * t1030 - t1194 - 0.12315e-2_f64 * t1053;
    let t1198 = t484 * t484;
    let t1199 = 1.0_f64 / t1198;
    let t1200 = t481 * t1199;
    let t1201 = 0.72691666666666666667e3_f64 * t1023;
    let t1203 = 0.78666666666666666667e2_f64 * t1049;
    let t1205 = -t1201 - 0.72691666666666666667e3_f64 * t1030 - t1203 - 0.78666666666666666667e2_f64 * t1053;
    (t1188, t1196, t1198, t1199, t1200, t1205)
}
