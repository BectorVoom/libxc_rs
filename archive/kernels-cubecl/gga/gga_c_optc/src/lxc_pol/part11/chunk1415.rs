//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1415/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1415<F: Float>(t26599: F, t26600: F, t52395: F, t58375: F, t58378: F, t58381: F, t58384: F, t58388: F, t58392: F, t58397: F, t58401: F, t58405: F, t58409: F, t58412: F) -> F {
    let t59294 = -F::cast_from(0.27545333333333333332e1_f64) * t52395 + t26599 + t26600 - F::cast_from(0.34431666666666666667e1_f64) * t58375 + F::cast_from(0.123954e2_f64) * t58378 - F::cast_from(0.13772666666666666667e1_f64) * t58381 - F::cast_from(0.185931e2_f64) * t58384 + F::cast_from(0.55570666666666666666e0_f64) * t58388 + F::cast_from(0.62517e0_f64) * t58392 + F::cast_from(0.68863333333333333334e1_f64) * t58397 - F::cast_from(0.123954e2_f64) * t58401 - F::cast_from(0.103295e1_f64) * t58405 + F::cast_from(0.123954e2_f64) * t58409 + F::cast_from(0.309885e1_f64) * t58412;
    t59294
}
