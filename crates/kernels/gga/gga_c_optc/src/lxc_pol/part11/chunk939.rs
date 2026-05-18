//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 939/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk939<F: Float>(t1038: F, t17380: F, t1450: F, t5126: F, t8617: F, t17344: F, t2869: F, t25: F) -> (F, F, F, F, F) {
    let t17381 = t1038 * t17380;
    let t17383 = t5126 * t1450;
    let t17384 = t8617 * t17383;
    let t17388 = t2869 * t17344;
    let t17389 = t25 * t17388;
    (t17381, t17383, t17384, t17388, t17389)
}
