//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 717/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk717<F: Float>(t1086: F, t989: F, t378: F, t994: F, t1071: F, t359: F, t3140: F, t3143: F) -> (F, F, F, F, F) {
    let t3278 = t989 * t1086;
    let t3286 = t1086 * t378;
    let t3287 = t994 * t3286;
    let t3291 = t359 * t1071;
    let t3298 = t3140 * t3143;
    (t3278, t3286, t3287, t3291, t3298)
}
