//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1092/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1092<F: Float>(t2668: F, t2674: F, t40326: F, t2595: F, t4941: F, t19: F, t4961: F, t7481: F, t7433: F, t123: F, t769: F, t4971: F, t7878: F) -> (F, F, F, F, F, F) {
    let t40328 = t2668 * t40326 * t2674;
    let t40355 = t2595 * t4941;
    let t40356 = t40355 * t19;
    let t40391 = t7481 * t4961;
    let t40480 = t7433 * t4961;
    let t40489 = t123 * t769;
    let t40526 = t7878 * t4971;
    (t40328, t40356, t40391, t40480, t40489, t40526)
}
