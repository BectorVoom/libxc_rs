//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 957/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk957<F: Float>(t1493: F, t36: F, t606: F, t119457: F, t60221: F, t8435: F, t13272: F, t32141: F, t33612: F, t644: F, t8621: F, t6972: F, t640: F, t7705: F, t1469: F, t8441: F) -> (F, F, F, F, F, F, F) {
    let t125279 = t1493 * t36 * t606;
    let t125280 = t119457 * t125279;
    let t125283 = t60221 * t8435;
    let t125286 = t13272 * t32141;
    let t125290 = t8621 * t33612 * t644;
    let t125294 = t8621 * t33612 * t6972;
    let t125298 = t8621 * t7705 * t640;
    let t125305 = t8621 * t8441 * t606 * t1469;
    (t125280, t125283, t125286, t125290, t125294, t125298, t125305)
}
