//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 626/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk626(t2953: f64, t2963: f64, t4068: f64, t4117: f64, t5108: f64, t5112: f64, t5115: f64, t5127: f64, t5134: f64, t5140: f64, t5142: f64, t5146: f64, t5149: f64, t5152: f64) -> f64 {
    let t5154 = -0.17648625e1_f64 * t5127 + 0.3529725e1_f64 * t5134 + t2953 + 0.34431666666666666666e0_f64 * t4068 - 0.34431666666666666667e0_f64 * t5108 + 0.103295e1_f64 * t5112 - 0.516475e0_f64 * t5115 + 0.31558125e0_f64 * t5140 + 0.6311625e0_f64 * t5142 + t2963 + 0.13892666666666666667e0_f64 * t4117 - 0.34731666666666666667e-1_f64 * t5146 + 0.20839e0_f64 * t5149 - 0.104195e0_f64 * t5152;
    t5154
}
