//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 832/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk832<F: Float>(t285: F, t7906: F, t24: F, t2629: F, t862: F, t2634: F, t6541: F, t865: F, t322: F, t7253: F, t7256: F, t6534: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7907 = t7906 * t285;
    let t7914 = t24 * t2629;
    let t7915 = t862 * t7914;
    let t7917 = t24 * t2634;
    let t7918 = t862 * t7917;
    let t7920 = t865 * t6541;
    let t7921 = t322 * t7920;
    let t7924 = t7253 * t7256;
    let t7925 = t7924 * t6534;
    (t7907, t7914, t7915, t7917, t7918, t7920, t7921, t7924, t7925)
}
