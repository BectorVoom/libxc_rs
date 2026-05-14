//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 823/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk823<F: Float>(t275: F, t8378: F, t176: F, t2548: F, t8: F, t191: F, t2264: F, t2436: F, t2566: F, t960: F, t2568: F, t339: F, t2433: F, t277: F, t364: F, t7623: F, t7626: F, t7628: F, t7631: F, t7666: F, t7675: F, t7678: F, t7684: F, t7688: F, t7691: F, t7694: F, t7698: F, t7726: F, t7833: F, t8304: F, t8307: F, t95: F, sigma0: F) -> (F, F, F, F, F, F, F) {
    let t8379 = t8378 * t275;
    let t8381 = t176 * t8379 * sigma0;
    let t8384 = t8 * t2548;
    let t8385 = t8384 * t191;
    let t8386 = t2436 * t2264;
    let t8387 = t8385 * t8386;
    let t8390 = t2566 * t960;
    let t8393 = 1.0 / t2568 / t339;
    let t8397 = -8.0 / 3.0 * t8304 + 8.0 / 9.0 * t8307 + t8381 * t364 / 2.0 + t7688 - t7694 - t7698 + 200.0 / 81.0 * t2433 * t8387 + 0.51689762869806860992e-2 * t95 * t277 * t8390 * t8393 - t7726 + t7833 + t7623 + t7626 + t7628 + t7631 + t7666 + t7675 - t7678 - t7684 + t7691;
    (t8381, t8384, t8385, t8386, t8387, t8393, t8397)
}
