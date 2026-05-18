//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 670/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk670<F: Float>(t4355: F, t4356: F, t1027: F, t3117: F, t441: F, t442: F, t140: F, t309: F, t446: F) -> (F, F, F) {
    let t4357 = t4355 * t4356;
    let t4374 = t3117 * t1027;
    let t4383 = t441 * t442;
    let t4385 = t446 * t309 * t140;
    let t4386 = t4383 * t4385;
    (t4357, t4374, t4386)
}
