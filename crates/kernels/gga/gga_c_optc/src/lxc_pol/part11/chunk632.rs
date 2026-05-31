//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 632/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk632<F: Float>(t1094: F, t5202: F, t3041: F, t3048: F, t4068: F, t4117: F, t5108: F, t5112: F, t5115: F, t5127: F, t5134: F, t5140: F, t5142: F, t5146: F, t5149: F, t5152: F) -> (F, F) {
    let t5203 = t5202 * t1094;
    let t5218 = -F::cast_from(0.1294625e1_f64) * t5127 + F::cast_from(0.258925e1_f64) * t5134 + t3041 + F::cast_from(0.20128333333333333334e0_f64) * t4068 - F::cast_from(0.20128333333333333333e0_f64) * t5108 + F::cast_from(0.60385e0_f64) * t5112 - F::cast_from(0.301925e0_f64) * t5115 + F::cast_from(0.82524375e-1_f64) * t5140 + F::cast_from(0.16504875e0_f64) * t5142 + t3048 + F::cast_from(0.11038e0_f64) * t4117 - F::cast_from(0.27595e-1_f64) * t5146 + F::cast_from(0.16557e0_f64) * t5149 - F::cast_from(0.82785e-1_f64) * t5152;
    (t5203, t5218)
}
