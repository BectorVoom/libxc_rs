//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 309/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk309<F: Float>(t1470: F, t70: F, t1469: F, t48: F, t51: F, t53: F, t60: F, t44: F, t56: F, t61: F, t626: F, t38: F, t633: F, t637: F, rho1: F, sigma2: F) -> (F, F, F, F, F, F) {
    let t1471 = t1470 * t70;
    let t1474 = t48 * t1469;
    let t1477 = t51 * rho1;
    let t1479 = 1.0 / t53 / t1477;
    let t1480 = sigma2 * t1479;
    let t1483 = t60 * t1469;
    let t1486 = 5.0 / 6.0 * t44 * t1474 - 8.0 / 3.0 * t1480 * t61 - 5.0 / 6.0 * t56 * t1483 + t626;
    let t1487 = t38 * t1486;
    let t1490 = t633 * t1469;
    let t1491 = t637 * t1469;
    let t1493 = -4.0 / 3.0 * t1490 + 4.0 / 3.0 * t1491;
    (t1471, t1474, t1480, t1486, t1487, t1493)
}
