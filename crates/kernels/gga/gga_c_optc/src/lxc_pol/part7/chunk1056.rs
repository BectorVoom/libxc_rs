//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1056/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1056<F: Float>(t669: F, t6960: F, t2096: F, t2105: F, t664: F, t6976: F, t166: F, t6975: F, t145: F, t2107: F, t2189: F, t6787: F, t9896: F) -> (F, F, F, F, F, F, F) {
    let t22911 = t6960 * t669;
    let t22915 = t2096 * t2105;
    let t22922 = t664 * t6976;
    let t22932 = F::new(1.0) / t6975 / t166;
    let t22933 = t145 * t22932;
    let t22934 = t2107 * t2107;
    let t22942 = t2189 * t2189;
    let t22949 = t9896 * t6787;
    (t22911, t22915, t22922, t22933, t22934, t22942, t22949)
}
