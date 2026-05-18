//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1213/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1213<F: Float>(t25035: F, t893: F, t7354: F, t8152: F, t862: F, t10838: F, t7359: F, t2583: F, t7879: F, t3843: F, t898: F, t2649: F, t7878: F) -> (F, F, F, F, F, F, F) {
    let t25036 = t893 * t25035;
    let t25041 = t862 * t8152 * t7354;
    let t25044 = t862 * t10838 * t7359;
    let t25050 = t2583 * t7879;
    let t25052 = t3843 * t898;
    let t25053 = t893 * t25052;
    let t25055 = t7878 * t2649;
    (t25036, t25041, t25044, t25050, t25052, t25053, t25055)
}
