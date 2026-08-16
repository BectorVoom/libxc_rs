//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 479/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk479(t1469: f64, t60: f64, t1474: f64, t1480: f64, t44: f64, t56: f64, t61: f64, t626: f64, t38: f64, t633: f64, t637: f64) -> (f64, f64, f64, f64, f64) {
    let t1483 = t60 * t1469;
    let t1486 = 5.0_f64 / 6.0_f64 * t44 * t1474 - 8.0_f64 / 3.0_f64 * t1480 * t61 - 5.0_f64 / 6.0_f64 * t56 * t1483 + t626;
    let t1487 = t38 * t1486;
    let t1490 = t633 * t1469;
    let t1491 = t637 * t1469;
    let t1493 = -4.0_f64 / 3.0_f64 * t1490 + 4.0_f64 / 3.0_f64 * t1491;
    (t1486, t1487, t1490, t1491, t1493)
}
