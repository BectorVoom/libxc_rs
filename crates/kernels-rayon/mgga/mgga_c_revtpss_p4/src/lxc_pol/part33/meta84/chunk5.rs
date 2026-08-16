//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 540/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk540(t1480: f64, t344: f64, t1225: f64, t1469: f64, t1012: f64, t1770: f64, t225: f64) -> (f64, f64, f64, f64) {
    let t1778 = t1480 * t344;
    let t1781 = t1225 * t1469;
    let t1782 = t1012 * t1781;
    let t1785 = t1770 * t225;
    (t1778, t1781, t1782, t1785)
}
