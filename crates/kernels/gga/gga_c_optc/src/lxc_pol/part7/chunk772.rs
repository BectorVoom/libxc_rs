//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 772/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk772<F: Float>(t6534: F, t7397: F, t322: F, t530: F, t866: F, t862: F, t2548: F, t7298: F, t2573: F, t861: F, t2623: F, t2626: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7398 = t7397 * t6534;
    let t7399 = t322 * t7398;
    let t7402 = t530 * t866;
    let t7403 = t862 * t7402;
    let t7405 = t2548 * t7298;
    let t7406 = t7405 * t6534;
    let t7407 = t322 * t7406;
    let t7410 = t2573 * t861;
    let t7413 = t2623 * t2626;
    (t7398, t7399, t7402, t7403, t7405, t7406, t7407, t7410, t7413)
}
