//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 821/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk821<F: Float>(t972: F, t346: F, t2302: F, t979: F, t2315: F, t7592: F, t7529: F, t7538: F, t7541: F, t7544: F, t7547: F, t7560: F, t7563: F, t7566: F, t7596: F, t7599: F) -> (F, F, F, F, F, F) {
    let t8343 = t972 * t972;
    let t8344 = 1.0 / t8343;
    let t8345 = t346 * t8344;
    let t8346 = t2302 * t979;
    let t8349 = t979 * t2315;
    let t8362 = 0.34962962962962962963e3 * t7592;
    let t8363 = -0.31466666666666666667e3 * t7560 + 0.15733333333333333333e3 * t7563 - 0.78666666666666666666e2 * t7596 - 0.47199999999999999999e3 * t7566 + 0.47199999999999999999e3 * t7599 - 0.14538333333333333333e4 * t7529 + 0.29076666666666666666e4 * t7538 - 0.14538333333333333333e4 * t7541 - 0.43614999999999999999e4 * t7544 + 0.43614999999999999999e4 * t7547 - t8362;
    (t8343, t8344, t8345, t8346, t8349, t8363)
}
