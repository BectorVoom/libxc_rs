//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 632/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk632(t1094: f64, t5202: f64, t3041: f64, t3048: f64, t4068: f64, t4117: f64, t5108: f64, t5112: f64, t5115: f64, t5127: f64, t5134: f64, t5140: f64, t5142: f64, t5146: f64, t5149: f64, t5152: f64) -> (f64, f64) {
    let t5203 = t5202 * t1094;
    let t5218 = -0.1294625e1_f64 * t5127 + 0.258925e1_f64 * t5134 + t3041 + 0.20128333333333333334e0_f64 * t4068 - 0.20128333333333333333e0_f64 * t5108 + 0.60385e0_f64 * t5112 - 0.301925e0_f64 * t5115 + 0.82524375e-1_f64 * t5140 + 0.16504875e0_f64 * t5142 + t3048 + 0.11038e0_f64 * t4117 - 0.27595e-1_f64 * t5146 + 0.16557e0_f64 * t5149 - 0.82785e-1_f64 * t5152;
    (t5203, t5218)
}
