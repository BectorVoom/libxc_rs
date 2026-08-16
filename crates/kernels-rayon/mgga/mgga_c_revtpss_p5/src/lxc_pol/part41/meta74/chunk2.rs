//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 448/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk448(t1486: f64, t38: f64, t1469: f64, t633: f64, t637: f64, t77: f64) -> (f64, f64) {
    let t1487 = t38 * t1486;
    let t1490 = t633 * t1469;
    let t1491 = t637 * t1469;
    let t1493 = -4.0_f64 / 3.0_f64 * t1490 + 4.0_f64 / 3.0_f64 * t1491;
    let t1494 = t77 * t1493;
    (t1487, t1494)
}
