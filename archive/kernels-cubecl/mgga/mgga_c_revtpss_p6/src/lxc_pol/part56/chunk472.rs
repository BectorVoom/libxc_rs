//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 472/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk472<F: Float>(t271: F, t905: F, t1071: F, t342: F, t1077: F, t384: F, t225: F, t1086: F, t989: F, t378: F, t994: F, t359: F) -> (F, F, F, F, F, F) {
    let t3252 = F::cast_from(1.0_f64) / t271 / t905;
    let t3264 = t342 * t1071;
    let t3268 = F::cast_from(1.0_f64) / t1077 / t384;
    let t3269 = t225 * t3268;
    let t3278 = t989 * t1086;
    let t3286 = t1086 * t378;
    let t3287 = t994 * t3286;
    let t3291 = t359 * t1071;
    (t3252, t3264, t3269, t3278, t3287, t3291)
}
