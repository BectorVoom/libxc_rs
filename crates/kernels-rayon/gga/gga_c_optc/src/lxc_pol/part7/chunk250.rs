//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 250/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk250(t103: f64, t193: f64, t197: f64, t745: f64, t102: f64, t195: f64, t616: f64) -> (f64, f64, f64) {
    let t749 = 100.0_f64 / 27.0_f64 * t193 * t745 * t103 * t197;
    let t750 = t195 * t102;
    let t751 = t197 * t616;
    let t755 = t749 - 25.0_f64 / 9.0_f64 * t193 * t750 * t751;
    (t750, t751, t755)
}
