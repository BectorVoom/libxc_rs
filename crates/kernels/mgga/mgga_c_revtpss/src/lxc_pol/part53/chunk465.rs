//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 465/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk465<F: Float>(t1086: F, t989: F, t378: F, t994: F, t1071: F, t359: F, t3140: F, t3143: F, t342: F, t335: F, t368: F, t1035: F, t389: F, t1941: F, t268: F, t404: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t3278 = t989 * t1086;
    let t3286 = t1086 * t378;
    let t3287 = t994 * t3286;
    let t3291 = t359 * t1071;
    let t3298 = t3140 * t3143;
    let t3299 = t342 * t3298;
    let t3302 = 1.0 / t368 / t335;
    let t3316 = t3140 * t1035;
    let t3317 = t342 * t3316;
    let t3335 = t389 * t389;
    let t3336 = 1.0 / t3335;
    let t3356 = t268 * t1941 * t404;
    (t3278, t3287, t3291, t3298, t3299, t3302, t3316, t3317, t3335, t3336, t3356)
}
