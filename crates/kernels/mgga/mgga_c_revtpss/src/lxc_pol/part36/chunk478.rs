//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 478/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk478<F: Float>(t3204: F, t366: F, t1014: F, t2857: F, t271: F, t905: F, t2852: F, t1077: F, t384: F, t225: F, t1086: F, t378: F, t994: F, t3140: F, t3143: F, t342: F) -> (F, F, F, F, F, F, F, F, F) {
    let t3205 = t3204 * t366;
    let t3236 = t1014 * t2857;
    let t3252 = 1.0 / t271 / t905;
    let t3253 = t3252 * t2852;
    let t3268 = 1.0 / t1077 / t384;
    let t3269 = t225 * t3268;
    let t3286 = t1086 * t378;
    let t3287 = t994 * t3286;
    let t3298 = t3140 * t3143;
    let t3299 = t342 * t3298;
    (t3205, t3236, t3252, t3253, t3269, t3286, t3287, t3298, t3299)
}
