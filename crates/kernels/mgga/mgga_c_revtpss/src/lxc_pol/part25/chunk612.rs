//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 612/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk612<F: Float>(t1150: F, t3427: F, t1131: F, t1129: F, t408: F, t421: F, t3385: F, t3356: F, t3358: F, t3365: F, t3370: F, t3374: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t3428 = t3427 * t1150;
    let t3430 = F::new(1.0) * t1131 * t3428;
    let t3431 = t1129 * t1129;
    let t3432 = F::new(1.0) / t3431;
    let t3433 = t408 * t3432;
    let t3434 = t421 * t421;
    let t3435 = F::new(1.0) / t3434;
    let t3436 = t3385 * t3435;
    let t3438 = F::new(0.16081979498692535067e2) * t3433 * t3436;
    let t3439 = F::new(0.22831111111111111111e-1) * t3356;
    let t3444 = t3439 - F::new(0.11415555555555555555e-1) * t3358 - F::new(0.11415555555555555555e-1) * t3365 + F::new(0.34246666666666666666e-1) * t3370 + F::new(0.17123333333333333333e-1) * t3374;
    (t3428, t3430, t3431, t3432, t3433, t3434, t3435, t3436, t3438, t3444)
}
