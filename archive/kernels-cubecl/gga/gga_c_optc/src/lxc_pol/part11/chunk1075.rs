//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1075/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1075<F: Float>(t1162: F, t1502: F, t3902: F, t8414: F, t8459: F, t1545: F, t8529: F, t1533: F, t9091: F, t1179: F, t35576: F, t191: F, t35363: F) -> (F, F, F, F, F, F) {
    let t35887 = t1162 * t3902 * t1502;
    let t35932 = t8459 * t8414;
    let t36182 = t1545 * t8529;
    let t36566 = t1533 * t9091;
    let t36641 = t1179 * t35576;
    let t36845 = t35363 * t191;
    (t35887, t35932, t36182, t36566, t36641, t36845)
}
