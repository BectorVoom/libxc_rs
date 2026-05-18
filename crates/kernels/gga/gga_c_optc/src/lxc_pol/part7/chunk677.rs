//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 677/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk677<F: Float>(t1917: F, t737: F, t1871: F, t558: F, t40: F, t1820: F, t1828: F, t1859: F, t1867: F, t586: F, t1757: F, t1784: F, t535: F) -> (F, F, F, F, F, F, F) {
    let t6337 = t737 * t1917;
    let t6340 = t558 * t1871;
    let t6341 = t40 * t6340;
    let t6342 = F::new(3.0) * t6341;
    let t6343 = t1820 * t1828;
    let t6347 = t1859 * t1867;
    let t6348 = t6347 * t586;
    let t6356 = F::new(6.0) * t1757 * t535 * t1784;
    (t6337, t6340, t6342, t6343, t6347, t6348, t6356)
}
