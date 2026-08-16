//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 687/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk687(t1086: f64, t989: f64, t378: f64, t994: f64, t1071: f64, t359: f64, t3140: f64, t3143: f64) -> (f64, f64, f64, f64, f64) {
    let t3278 = t989 * t1086;
    let t3286 = t1086 * t378;
    let t3287 = t994 * t3286;
    let t3291 = t359 * t1071;
    let t3298 = t3140 * t3143;
    (t3278, t3286, t3287, t3291, t3298)
}
