//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 609/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk609<F: Float>(t2953: F, t2963: F, t4068: F, t4117: F, t5108: F, t5112: F, t5115: F, t5127: F, t5134: F, t5140: F, t5142: F, t5146: F, t5149: F, t5152: F) -> (F,) {
    let t5154 = -0.17648625e1 * t5127 + 0.3529725e1 * t5134 + t2953 + 0.34431666666666666666e0 * t4068 - 0.34431666666666666667e0 * t5108 + 0.103295e1 * t5112 - 0.516475e0 * t5115 + 0.31558125e0 * t5140 + 0.6311625e0 * t5142 + t2963 + 0.13892666666666666667e0 * t4117 - 0.34731666666666666667e-1 * t5146 + 0.20839e0 * t5149 - 0.104195e0 * t5152;
    (t5154,)
}
