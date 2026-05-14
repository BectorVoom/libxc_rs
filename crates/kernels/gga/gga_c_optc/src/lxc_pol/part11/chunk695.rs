//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 695/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk695<F: Float>(t8285: F, t93: F, t7592: F, t7523: F, t972: F, t346: F, t2548: F, t8: F, t2568: F, t339: F, t2848: F, t50: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t8287 = 1.0 / t8285 * t93;
    let t8319 = 0.54733333333333333333e-2 * t7592;
    let t8321 = 0.60319259259259259259e1 * t7523;
    let t8343 = t972 * t972;
    let t8344 = 1.0 / t8343;
    let t8345 = t346 * t8344;
    let t8362 = 0.34962962962962962963e3 * t7592;
    let t8364 = 0.22615185185185185185e4 * t7523;
    let t8384 = t8 * t2548;
    let t8393 = 1.0 / t2568 / t339;
    let t8414 = 1.0 / t2848 / t50;
    (t8287, t8319, t8321, t8343, t8344, t8345, t8362, t8364, t8384, t8393, t8414)
}
