//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 636/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk636(t1141: f64, t1146: f64, t1145: f64, t469: f64, t454: f64, t1182: f64) -> (f64, f64, f64, f64) {
    let t3164 = t1141 * t1146;
    let t3169 = 1.0_f64 / t1145 / t469;
    let t3170 = t454 * t3169;
    let t3171 = t1182 * t1182;
    (t3164, t3169, t3170, t3171)
}
