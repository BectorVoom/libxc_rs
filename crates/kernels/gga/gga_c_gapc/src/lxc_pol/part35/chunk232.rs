//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 232/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk232<F: Float>(t298: F, t891: F, t181: F, t282: F, t6: F, t481: F, t311: F, t315: F, t435: F, t122: F, t188: F) -> (F, F, F, F, F, F, F, F, F) {
    let t892 = t298 * t891;
    let t893 = t181 * t892;
    let t896 = t282 * t6;
    let t897 = t896 * t481;
    let t898 = t311 * t897;
    let t899 = t435 * t315;
    let t902 = t282 * t122;
    let t903 = t902 * t188;
    let t904 = t311 * t903;
    (t892, t893, t896, t897, t898, t899, t902, t903, t904)
}
