//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 748/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk748<F: Float>(t2107: F, t8717: F, t2014: F, t1932: F, t2007: F, t2052: F, t2056: F, t2089: F, t2108: F, t508: F, t569: F, t651: F, t6985: F, t8463: F, t8568: F, t8627: F, t8630: F, t8636: F, t8637: F, t8643: F, t8687: F, t8695: F, t8699: F, t8716: F) -> (F, F) {
    let t8718 = t2107 * t8717;
    let t8719 = t2014 * t8718;
    let t8720 = -t1932 * t2089 - t2007 * t2052 - 2.0 * t2056 * t6985 + t2108 * t8568 - t508 * t8627 + t569 * t8695 - 2.0 * t651 * t8637 - t8463 - t8630 - t8636 - t8643 - t8687 + t8699 + t8716 - t8719;
    (t8718, t8720)
}
