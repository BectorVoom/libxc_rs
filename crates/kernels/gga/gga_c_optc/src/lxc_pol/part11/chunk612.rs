//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 612/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk612<F: Float>(t1056: F, t5186: F, t1037: F, t3020: F, t5170: F, t3018: F, t3024: F, t4068: F, t5108: F, t5112: F, t5115: F) -> (F, F, F, F, F) {
    let t5187 = t5186 * t1056;
    let t5189 = 1.0 * t1037 * t5187;
    let t5190 = t5170 * t3020;
    let t5192 = 0.16081824322151104822e2 * t3018 * t5190;
    let t5197 = t3024 + 0.61805555555555555556e-2 * t4068 - 0.61805555555555555555e-2 * t5108 + 0.18541666666666666667e-1 * t5112 - 0.92708333333333333333e-2 * t5115;
    (t5187, t5189, t5190, t5192, t5197)
}
