//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 663/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk663(t497: f64, t5440: f64, t2844: f64, t2866: f64, t4068: f64, t4117: f64, t5108: f64, t5112: f64, t5115: f64, t5146: f64, t5149: f64, t5152: f64) -> (f64, f64) {
    let t5441 = t5440 * t497;
    let t5454 = t2844 + 0.12925555555555555555e1_f64 * t4068 - 0.12925555555555555555e1_f64 * t5108 + 0.38776666666666666666e1_f64 * t5112 - 0.19388333333333333333e1_f64 * t5115 + t2866 + 0.1642e-2_f64 * t4117 - 0.4105e-3_f64 * t5146 + 0.2463e-2_f64 * t5149 - 0.12315e-2_f64 * t5152;
    (t5441, t5454)
}
