//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 531/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk531<F: Float>(t1133: F, t4380: F, t441: F, t442: F, t140: F, t309: F, t446: F) -> (F, F, F) {
    let t4381 = t1133 * t4380;
    let t4383 = t441 * t442;
    let t4385 = t446 * t309 * t140;
    let t4386 = t4383 * t4385;
    (t4381, t4383, t4386)
}
