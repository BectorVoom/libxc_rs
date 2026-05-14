//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 984/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk984<F: Float>(t8414: F, t8459: F, t1545: F, t8529: F, t1533: F, t9091: F, t1179: F, t35576: F, t191: F, t35363: F, t1113: F, t529: F, t1561: F, t9303: F, t2229: F, t4759: F) -> (F, F, F, F, F, F, F, F) {
    let t35932 = t8459 * t8414;
    let t36182 = t1545 * t8529;
    let t36566 = t1533 * t9091;
    let t36641 = t1179 * t35576;
    let t36845 = t35363 * t191;
    let t36863 = t529 * t1113 * t191;
    let t36985 = t1561 * t9303;
    let t37138 = t2229 * t4759;
    (t35932, t36182, t36566, t36641, t36845, t36863, t36985, t37138)
}
