//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 629/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk629(t1056: f64, t5170: f64, t2993: f64, t3000: f64, t3007: f64, t4068: f64, t4117: f64, t5108: f64, t5112: f64, t5115: f64, t5127: f64, t5134: f64, t5140: f64, t5142: f64, t5146: f64, t5149: f64, t5152: f64) -> (f64, f64, f64) {
    let t5171 = t5170 * t1056;
    let t5173 = 2.0_f64 * t2993 * t5171;
    let t5186 = -0.9494625e0_f64 * t5127 + 0.1898925e1_f64 * t5134 + t3000 + 0.19931111111111111111e0_f64 * t4068 - 0.19931111111111111111e0_f64 * t5108 + 0.59793333333333333334e0_f64 * t5112 - 0.29896666666666666667e0_f64 * t5115 + 0.15358125e0_f64 * t5140 + 0.3071625e0_f64 * t5142 + t3007 + 0.10954222222222222222e0_f64 * t4117 - 0.27385555555555555556e-1_f64 * t5146 + 0.16431333333333333333e0_f64 * t5149 - 0.82156666666666666667e-1_f64 * t5152;
    (t5171, t5173, t5186)
}
