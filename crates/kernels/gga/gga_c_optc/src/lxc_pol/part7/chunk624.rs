//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 624/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk624<F: Float>(t3218: F, t438: F, t458: F, t3126: F, t429: F, t914: F, t3093: F, t1168: F, t442: F, t1120: F) -> (F, F, F, F, F, F, F) {
    let t3219 = t3218 * t438;
    let t3220 = t458 * t3219;
    let t3224 = t429 * t3126 * t438;
    let t3225 = t914 * t3224;
    let t3230 = t914 * t3093;
    let t3233 = t1168 * t442;
    let t3234 = t3233 * t1120;
    (t3219, t3220, t3224, t3225, t3230, t3233, t3234)
}
