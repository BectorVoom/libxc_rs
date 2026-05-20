//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1076/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1076<F: Float>(t3357: F, t5044: F, t6423: F, t6427: F, t6431: F, t422: F, t1733: F, t5063: F, t1732: F, t1150: F, t3384: F, t1723: F) -> (F, F, F, F, F, F, F) {
    let t6433 = t3357 - F::cast_from(0.11872222222222222222e-1_f64) * t5044 - F::cast_from(0.11872222222222222222e-1_f64) * t6423 + F::cast_from(0.35616666666666666666e-1_f64) * t6427 + F::cast_from(0.17808333333333333333e-1_f64) * t6431;
    let t6435 = F::new(0.621814e-1) * t6433 * t422;
    let t6437 = F::new(2.0) * t5063 * t1733;
    let t6438 = t1732 * t1732;
    let t6439 = t6438 * t1150;
    let t6441 = F::new(2.0) * t3384 * t6439;
    let t6442 = t1723 * t1723;
    (t6433, t6435, t6437, t6438, t6439, t6441, t6442)
}
