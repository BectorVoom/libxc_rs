//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 822/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk822(t2107: f64, t8717: f64, t2014: f64, t1932: f64, t2007: f64, t2052: f64, t2056: f64, t2089: f64, t2108: f64, t508: f64, t569: f64, t651: f64, t6985: f64, t8463: f64, t8568: f64, t8627: f64, t8630: f64, t8636: f64, t8637: f64, t8643: f64, t8687: f64, t8695: f64, t8699: f64, t8716: f64) -> (f64, f64) {
    let t8718 = t2107 * t8717;
    let t8719 = t2014 * t8718;
    let t8720 = -t1932 * t2089 - t2007 * t2052 - 2.0_f64 * t2056 * t6985 + t2108 * t8568 - t508 * t8627 + t569 * t8695 - 2.0_f64 * t651 * t8637 - t8463 - t8630 - t8636 - t8643 - t8687 + t8699 + t8716 - t8719;
    (t8718, t8720)
}
