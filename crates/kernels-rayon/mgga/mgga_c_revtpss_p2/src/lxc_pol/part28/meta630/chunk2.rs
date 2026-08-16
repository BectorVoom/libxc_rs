//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2274/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2274(t28283: f64, t571: f64, t28234: f64, t575: f64, t101558: f64, t101563: f64, t101609: f64, t101651: f64, t1456: f64, t1458: f64, t1914: f64, t1921: f64, t26094: f64, t26133: f64, t3: f64, t4168: f64, t5808: f64, t7319: f64, t7940: f64, t92559: f64, t92563: f64, t95127: f64) -> f64 {
    let t101656 = 2.0_f64 * t571 * t28283;
    let t101658 = 2.0_f64 * t28234 * t575;
    let t101659 = t26094 * t1921 + 2.0_f64 * t1456 * t28283 + t3 * t101558 * t575 + t7940 * t4168 + t95127 + t101563 + 2.0_f64 * t7319 * t5808 + t1914 * t26133 + t92563 + t1458 * (t101609 + t101651) + 2.0_f64 * t92559 + t101656 + t101658;
    t101659
}
