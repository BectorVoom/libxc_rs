//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 665/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk665<F: Float>(t2354: F, t33341: F, t684: F, t6118: F, t713: F, t7484: F, t2506: F, t1434: F, t193: F, t202: F, t7446: F, t237: F, t17839: F, t218: F, t679: F, t689: F, sigma2: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t33343 = t2354 * t33341 * t684;
    let t33344 = t6118 * t33343;
    let t33346 = t7484 * t713;
    let t33347 = t2506 * t33346;
    let t33349 = t1434 * t193 * t33347;
    let t33350 = t202 * t7446;
    let t33351 = t33350 * t237;
    let t33356 = t17839 * sigma2;
    let t33357 = t218 * t679;
    let t33359 = t33356 * t33357 * t689;
    (t33343, t33344, t33346, t33347, t33349, t33350, t33351, t33356, t33357, t33359)
}
