//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 606/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk606<F: Float>(t1038: F, t5133: F, t2958: F, t5126: F, t1045: F, t2869: F, t5101: F, t25: F) -> (F, F, F, F, F) {
    let t5134 = t1038 * t5133;
    let t5140 = t2958 * t5126;
    let t5142 = t1045 * t5133;
    let t5145 = t2869 * t5101;
    let t5146 = t25 * t5145;
    (t5134, t5140, t5142, t5145, t5146)
}
