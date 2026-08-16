//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 677/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk677(t1917: f64, t737: f64, t1871: f64, t558: f64, t40: f64, t1820: f64, t1828: f64, t1859: f64, t1867: f64, t586: f64, t1757: f64, t1784: f64, t535: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6337 = t737 * t1917;
    let t6340 = t558 * t1871;
    let t6341 = t40 * t6340;
    let t6342 = 3.0_f64 * t6341;
    let t6343 = t1820 * t1828;
    let t6347 = t1859 * t1867;
    let t6348 = t6347 * t586;
    let t6356 = 6.0_f64 * t1757 * t535 * t1784;
    (t6337, t6340, t6342, t6343, t6347, t6348, t6356)
}
