//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1075/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1075(t1162: f64, t1502: f64, t3902: f64, t8414: f64, t8459: f64, t1545: f64, t8529: f64, t1533: f64, t9091: f64, t1179: f64, t35576: f64, t191: f64, t35363: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t35887 = t1162 * t3902 * t1502;
    let t35932 = t8459 * t8414;
    let t36182 = t1545 * t8529;
    let t36566 = t1533 * t9091;
    let t36641 = t1179 * t35576;
    let t36845 = t35363 * t191;
    (t35887, t35932, t36182, t36566, t36641, t36845)
}
