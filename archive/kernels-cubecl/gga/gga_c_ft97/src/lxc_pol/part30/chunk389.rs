//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 389/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk389<F: Float>(t2665: F, t6318: F, t684: F, t6317: F, t2781: F, t6278: F, t1486: F, t193: F, t6260: F, t852: F, t1491: F, t375: F, t89: F) -> (F, F, F, F, F, F, F) {
    let t6320 = t2665 * t6318 * t684;
    let t6321 = t6317 * t6320;
    let t6323 = t2781 * t6278;
    let t6325 = t1486 * t193 * t6323;
    let t6327 = t852 * t6260;
    let t6329 = t1486 * t193 * t6327;
    let t6332 = t89 * t375 * t1491;
    (t6320, t6321, t6323, t6325, t6327, t6329, t6332)
}
