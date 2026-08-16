//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 487/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk487<F: Float>(t225: F, t3268: F, t1086: F, t378: F, t994: F, t3140: F, t3143: F, t342: F, t335: F, t368: F) -> (F, F, F, F, F, F) {
    let t3269 = t225 * t3268;
    let t3286 = t1086 * t378;
    let t3287 = t994 * t3286;
    let t3298 = t3140 * t3143;
    let t3299 = t342 * t3298;
    let t3302 = F::cast_from(1.0_f64) / t368 / t335;
    (t3269, t3286, t3287, t3298, t3299, t3302)
}
