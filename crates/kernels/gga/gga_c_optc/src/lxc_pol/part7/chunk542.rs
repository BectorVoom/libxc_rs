//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 542/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk542<F: Float>(t2367: F, t916: F, t913: F, t909: F, t282: F, t911: F, t115: F, sigma0: F) -> (F, F, F, F, F) {
    let t2715 = t2367 * t916;
    let t2716 = t913 * t2715;
    let t2718 = t909 * sigma0;
    let t2719 = t282 * t911;
    let t2720 = t2719 * t115;
    let t2721 = t2718 * t2720;
    (t2715, t2716, t2718, t2719, t2721)
}
