//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 630/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk630(t1056: f64, t5186: f64, t1037: f64, t3020: f64, t5170: f64, t3018: f64, t3024: f64, t4068: f64, t5108: f64, t5112: f64, t5115: f64) -> (f64, f64, f64, f64, f64) {
    let t5187 = t5186 * t1056;
    let t5189 = 1.0_f64 * t1037 * t5187;
    let t5190 = t5170 * t3020;
    let t5192 = 0.16081824322151104822e2_f64 * t3018 * t5190;
    let t5197 = t3024 + 0.61805555555555555556e-2_f64 * t4068 - 0.61805555555555555555e-2_f64 * t5108 + 0.18541666666666666667e-1_f64 * t5112 - 0.92708333333333333333e-2_f64 * t5115;
    (t5187, t5189, t5190, t5192, t5197)
}
