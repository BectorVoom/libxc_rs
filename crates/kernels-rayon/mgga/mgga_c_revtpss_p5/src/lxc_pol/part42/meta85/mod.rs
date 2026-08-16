//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta85 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk498;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk499;
use chunk2::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk500;
use chunk3::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk501;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta85(t1480: f64, t344: f64, t1225: f64, t1469: f64, t1012: f64, t1770: f64, t225: f64, t480: f64, t1774: f64, t482: f64, t372: f64, t371: f64, t1721: f64, t1735: f64, t1761: f64, t1763: f64, t1767: f64, t1250: f64, t1042: f64, t476: f64, t51: f64, t52: f64, t475: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1778, t1781, t1782, t1785, t1786, t1789, t1791) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk498(t1480, t344, t1225, t1469, t1012, t1770, t225, t480, t1774, t482, t372, t371);
        let t1794 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk499(t1721, t1735, t1761, t1763, t1767);
        let (t1796, t1797) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk500(t1794, t482, t1250, t1042);
        let (t1802, t1803) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk501(t476, t51, t52, t475);
    (t1778, t1781, t1782, t1785, t1786, t1789, t1791, t1794, t1796, t1797, t1802, t1803)
}
