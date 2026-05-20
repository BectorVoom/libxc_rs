//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3080/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3080<F: Float>(t1678: F, t3043: F, t3259: F, t4746: F, t15885: F, t993: F, t378: F, t11223: F, t16163: F, t3169: F, t1041: F, t11262: F, t4868: F) -> (F, F, F, F, F, F, F) {
    let t53180 = t3043 * t1678;
    let t53208 = t4746 * t3259;
    let t53222 = t15885 * t993;
    let t53223 = t53222 * t378;
    let t53281 = t11223 * t1678;
    let t53290 = t3169 * t16163;
    let t53293 = t1041 * t11262 * t4868;
    (t53180, t53208, t53222, t53223, t53281, t53290, t53293)
}
