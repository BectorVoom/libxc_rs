//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 870/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk870<F: Float>(t2294: F, t973: F, t2300: F, t970: F, t972: F, t346: F, t2302: F, t979: F, t2315: F, t7592: F, t7529: F, t7538: F, t7541: F, t7544: F, t7547: F, t7560: F, t7563: F, t7566: F, t7596: F, t7599: F) -> (F, F, F, F, F, F, F, F) {
    let t8335 = t2294 * t973;
    let t8338 = t970 * t2300;
    let t8343 = t972 * t972;
    let t8344 = F::new(1.0) / t8343;
    let t8345 = t346 * t8344;
    let t8346 = t2302 * t979;
    let t8349 = t979 * t2315;
    let t8362 = F::cast_from(0.34962962962962962963e3_f64) * t7592;
    let t8363 = -F::cast_from(0.31466666666666666667e3_f64) * t7560 + F::cast_from(0.15733333333333333333e3_f64) * t7563 - F::cast_from(0.78666666666666666666e2_f64) * t7596 - F::cast_from(0.47199999999999999999e3_f64) * t7566 + F::cast_from(0.47199999999999999999e3_f64) * t7599 - F::cast_from(0.14538333333333333333e4_f64) * t7529 + F::cast_from(0.29076666666666666666e4_f64) * t7538 - F::cast_from(0.14538333333333333333e4_f64) * t7541 - F::cast_from(0.43614999999999999999e4_f64) * t7544 + F::cast_from(0.43614999999999999999e4_f64) * t7547 - t8362;
    (t8335, t8338, t8343, t8344, t8345, t8346, t8349, t8363)
}
