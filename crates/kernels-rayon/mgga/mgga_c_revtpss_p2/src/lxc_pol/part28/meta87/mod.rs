//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta87 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk547;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk548;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk549;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk550;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta87(t1790: f64, t371: f64, t1721: f64, t1735: f64, t1761: f64, t1763: f64, t1767: f64, t482: f64, t1250: f64, t1042: f64, t476: f64, t51: f64, t52: f64, t475: f64, t467: f64, t1264: f64, t1715: f64, t247: f64, t1221: f64, t1222: f64, t1235: f64, t1247: f64, t1258: f64, t1261: f64, t1778: f64, t1782: f64, t1786: f64, t464: f64, t484: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1791, t1794) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk547(t1790, t371, t1721, t1735, t1761, t1763, t1767);
        let (t1796, t1797, t1802) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk548(t1794, t482, t1250, t1042, t476, t51, t52);
        let t1803 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk549(t1802, t475);
        let (t1804, t1808, t1811) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk550(t1803, t467, t1264, t1715, t247, t1221, t1222, t1235, t1247, t1258, t1261, t1778, t1782, t1786, t1791, t1797, t464, t484);
    (t1791, t1794, t1796, t1797, t1802, t1803, t1804, t1808, t1811)
}
