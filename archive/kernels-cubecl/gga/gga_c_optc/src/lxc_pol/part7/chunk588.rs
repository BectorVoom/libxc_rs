//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 588/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk588<F: Float>(t2843: F, t2865: F, t2845: F, t2852: F, t2858: F, t2862: F, t2867: F, t2871: F, t2874: F, t2877: F) -> F {
    let t2890 = F::cast_from(0.96922222222222222222e3_f64) * t2843;
    let t2895 = F::cast_from(0.13111111111111111111e3_f64) * t2865;
    let t2900 = t2890 + F::cast_from(0.48461111111111111112e3_f64) * t2845 - F::cast_from(0.48461111111111111111e3_f64) * t2852 + F::cast_from(0.14538333333333333333e4_f64) * t2858 - F::cast_from(0.72691666666666666667e3_f64) * t2862 + t2895 + F::cast_from(0.10488888888888888889e3_f64) * t2867 - F::cast_from(0.26222222222222222222e2_f64) * t2871 + F::cast_from(0.15733333333333333333e3_f64) * t2874 - F::cast_from(0.78666666666666666667e2_f64) * t2877;
    t2900
}
