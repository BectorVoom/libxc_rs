//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 474/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk474(t1659: f64, t366: f64, t1651: f64, t373: f64, t372: f64, t371: f64, t1598: f64, t1612: f64, t1638: f64, t1640: f64, t1644: f64) -> (f64, f64, f64, f64) {
    let t1660 = t1659 * t366;
    let t1663 = t373 * t1651;
    let t1664 = t372 * t1663;
    let t1665 = t371 * t1664;
    let t1668 = -t1598 + t1612 + t1638 + t1640 - t1644;
    (t1660, t1663, t1665, t1668)
}
