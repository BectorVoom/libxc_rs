//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 439/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk439(t1663: f64, t197: f64, t1403: f64, t1661: f64, t587: f64, t708: f64, t723: f64, t1615: f64, t1619: f64, t1626: f64, t1629: f64, t1633: f64, t1637: f64, t1647: f64, t1650: f64, t1654: f64, t1658: f64) -> (f64, f64, f64, f64, f64) {
    let t1664 = t197 * t1663;
    let t1665 = t1664 * t1403;
    let t1666 = t1661 * t1665;
    let t1668 = 4.0_f64 / 27.0_f64 * t587 * t1666;
    let t1669 = t708 * t723;
    let t1671 = -4.0_f64 / 45.0_f64 * t1615 + t1619 - t1626 + t1629 + t1633 + t1637 + t1647 + t1650 + t1654 + t1658 + t1668 + 4.0_f64 / 9.0_f64 * t1669;
    (t1665, t1666, t1668, t1669, t1671)
}
