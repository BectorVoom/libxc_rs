//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 352/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk352(t469: f64, t454: f64, t19: f64, t463: f64, t427: f64, t911: f64) -> (f64, f64, f64, f64, f64) {
    let t1145 = t469 * t469;
    let t1146 = 1.0_f64 / t1145;
    let t1147 = t454 * t1146;
    let t1148 = t19 * t463;
    let t1150 = t1148 * t427 * t911;
    (t1145, t1146, t1147, t1148, t1150)
}
