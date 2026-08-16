//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 665/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk665<F: Float>(t2890: F, t2895: F, t4068: F, t4117: F, t5108: F, t5112: F, t5115: F, t5146: F, t5149: F, t5152: F) -> F {
    let t5469 = t2890 + F::cast_from(0.48461111111111111112e3_f64) * t4068 - F::cast_from(0.48461111111111111111e3_f64) * t5108 + F::cast_from(0.14538333333333333333e4_f64) * t5112 - F::cast_from(0.72691666666666666667e3_f64) * t5115 + t2895 + F::cast_from(0.10488888888888888889e3_f64) * t4117 - F::cast_from(0.26222222222222222222e2_f64) * t5146 + F::cast_from(0.15733333333333333333e3_f64) * t5149 - F::cast_from(0.78666666666666666667e2_f64) * t5152;
    t5469
}
