//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 582/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk582<F: Float>(t3357: F, t3358: F, t3365: F, t3370: F, t3374: F, t422: F, t1126: F, t1130: F, t1151: F, t1129: F, t418: F, t408: F) -> (F, F, F, F, F, F) {
    let t3376 = t3357 - F::cast_from(0.11872222222222222222e-1_f64) * t3358 - F::cast_from(0.11872222222222222222e-1_f64) * t3365 + F::cast_from(0.35616666666666666666e-1_f64) * t3370 + F::cast_from(0.17808333333333333333e-1_f64) * t3374;
    let t3378 = F::new(0.621814e-1) * t3376 * t422;
    let t3379 = t1126 * t1130;
    let t3381 = F::new(2.0) * t3379 * t1151;
    let t3382 = t1129 * t418;
    let t3383 = F::new(1.0) / t3382;
    let t3384 = t408 * t3383;
    (t3376, t3378, t3379, t3381, t3383, t3384)
}
