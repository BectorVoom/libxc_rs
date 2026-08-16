//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 472/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk472(t271: f64, t905: f64, t1071: f64, t342: f64, t1077: f64, t384: f64, t225: f64, t1086: f64, t989: f64, t378: f64, t994: f64, t359: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3252 = 1.0_f64 / t271 / t905;
    let t3264 = t342 * t1071;
    let t3268 = 1.0_f64 / t1077 / t384;
    let t3269 = t225 * t3268;
    let t3278 = t989 * t1086;
    let t3286 = t1086 * t378;
    let t3287 = t994 * t3286;
    let t3291 = t359 * t1071;
    (t3252, t3264, t3269, t3278, t3287, t3291)
}
