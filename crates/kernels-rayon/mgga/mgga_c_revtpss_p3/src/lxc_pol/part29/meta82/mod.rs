//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta82 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk506;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk507;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk508;
use chunk3::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk509;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta82(t1642: f64, t981: f64, t1594: f64, t986: f64, t341: f64, t997: f64, t996: f64, t1015: f64, t1469: f64, t1012: f64, t225: f64, t366: f64, t373: f64, t372: f64, t371: f64, t1598: f64, t1612: f64, t1638: f64, t1640: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1644, t1646, t1647) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk506(t1642, t981, t1594, t986, t341);
        let t1651 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk507(t1594, t997);
        let (t1652, t1655, t1656, t1659) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk508(t1651, t996, t1015, t1469, t1012, t1647, t225);
        let (t1660, t1663, t1665, t1668) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk509(t1659, t366, t1651, t373, t372, t371, t1598, t1612, t1638, t1640, t1644);
    (t1644, t1646, t1647, t1651, t1652, t1655, t1656, t1659, t1660, t1663, t1665, t1668)
}
