//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 592/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk592<F: Float>(t2994: F, t3020: F, t3018: F, t2843: F, t2845: F, t2852: F, t2858: F, t2862: F) -> (F, F, F) {
    let t3021 = t2994 * t3020;
    let t3023 = 0.16081824322151104822e2 * t3018 * t3021;
    let t3024 = 0.12361111111111111111e-1 * t2843;
    let t3029 = t3024 + 0.61805555555555555556e-2 * t2845 - 0.61805555555555555555e-2 * t2852 + 0.18541666666666666667e-1 * t2858 - 0.92708333333333333333e-2 * t2862;
    (t3021, t3023, t3029)
}
