//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 856/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk856<F: Float>(t2797: F, t2800: F, t7274: F, t866: F, t930: F, t288: F, t875: F, t2606: F, t3813: F, t2663: F, t277: F, t115: F) -> (F, F, F, F, F, F) {
    let t8180 = t2797 * t2800;
    let t8182 = t7274 * t866;
    let t8183 = t930 * t8182;
    let t8185 = t288 * t875;
    let t8186 = t3813 * t2606;
    let t8187 = t8185 * t8186;
    let t8191 = F::new(1.0) / t2663 / t277;
    let t8192 = t8191 * t115;
    (t8180, t8182, t8183, t8185, t8187, t8192)
}
