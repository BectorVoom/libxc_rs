//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 448/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk448<F: Float>(t1469: F, t36: F, t70: F, t48: F, t51: F, t53: F, rho1: F, sigma2: F) -> (F, F, F, F) {
    let t1470 = t36 * t1469;
    let t1471 = t1470 * t70;
    let t1474 = t48 * t1469;
    let t1477 = t51 * rho1;
    let t1479 = F::cast_from(1.0_f64) / t53 / t1477;
    let t1480 = sigma2 * t1479;
    (t1470, t1471, t1474, t1480)
}
