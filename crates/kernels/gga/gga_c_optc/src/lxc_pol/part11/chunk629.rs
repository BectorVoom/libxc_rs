//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 629/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk629<F: Float>(t1056: F, t5170: F, t2993: F, t3000: F, t3007: F, t4068: F, t4117: F, t5108: F, t5112: F, t5115: F, t5127: F, t5134: F, t5140: F, t5142: F, t5146: F, t5149: F, t5152: F) -> (F, F, F) {
    let t5171 = t5170 * t1056;
    let t5173 = F::cast_from(2.0_f64) * t2993 * t5171;
    let t5186 = -F::cast_from(0.9494625e0_f64) * t5127 + F::cast_from(0.1898925e1_f64) * t5134 + t3000 + F::cast_from(0.19931111111111111111e0_f64) * t4068 - F::cast_from(0.19931111111111111111e0_f64) * t5108 + F::cast_from(0.59793333333333333334e0_f64) * t5112 - F::cast_from(0.29896666666666666667e0_f64) * t5115 + F::cast_from(0.15358125e0_f64) * t5140 + F::cast_from(0.3071625e0_f64) * t5142 + t3007 + F::cast_from(0.10954222222222222222e0_f64) * t4117 - F::cast_from(0.27385555555555555556e-1_f64) * t5146 + F::cast_from(0.16431333333333333333e0_f64) * t5149 - F::cast_from(0.82156666666666666667e-1_f64) * t5152;
    (t5171, t5173, t5186)
}
