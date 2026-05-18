//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 873/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk873<F: Float>(t275: F, t8378: F, t176: F, t2548: F, t8: F, t191: F, t2264: F, t2436: F, t2566: F, t960: F, t2568: F, t339: F, sigma0: F) -> (F, F, F, F, F, F, F) {
    let t8379 = t8378 * t275;
    let t8381 = t176 * t8379 * sigma0;
    let t8384 = t8 * t2548;
    let t8385 = t8384 * t191;
    let t8386 = t2436 * t2264;
    let t8387 = t8385 * t8386;
    let t8390 = t2566 * t960;
    let t8393 = F::new(1.0) / t2568 / t339;
    (t8381, t8384, t8385, t8386, t8387, t8390, t8393)
}
