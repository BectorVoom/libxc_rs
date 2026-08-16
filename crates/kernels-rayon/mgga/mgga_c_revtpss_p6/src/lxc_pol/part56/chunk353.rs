//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 353/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk353(t1651: f64, t996: f64, t1015: f64, t1469: f64, t1012: f64, t1647: f64, t225: f64, t366: f64, t373: f64, t372: f64, t371: f64, t1598: f64, t1612: f64, t1638: f64, t1640: f64, t1644: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1652 = t996 * t1651;
    let t1655 = t1015 * t1469;
    let t1656 = t1012 * t1655;
    let t1659 = t1647 * t225;
    let t1660 = t1659 * t366;
    let t1663 = t373 * t1651;
    let t1664 = t372 * t1663;
    let t1665 = t371 * t1664;
    let t1668 = -t1598 + t1612 + t1638 + t1640 - t1644;
    (t1652, t1655, t1656, t1659, t1660, t1663, t1665, t1668)
}
