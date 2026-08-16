//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta84 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk535;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk536;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk537;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk538;
use chunk4::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk539;
use chunk5::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk540;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta84(t1161: f64, t1180: f64, t1721: f64, t1735: f64, t1737: f64, t1745: f64, t1750: f64, t1757: f64, t300: f64, t435: f64, t1179: f64, t1188: f64, t1756: f64, t1196: f64, t1201: f64, t1717: f64, t459: f64, t1212: f64, t1211: f64, t1480: f64, t344: f64, t1225: f64, t1469: f64, t1012: f64, t225: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1761, t1763, t1765) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk535(t1161, t1180, t1721, t1735, t1737, t1745, t1750, t1757, t300, t435, t1179, t1188, t1756);
        let (t1767, t1769) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk536(t1196, t1765, t1201, t1717);
        let t1770 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk537(t1769, t459);
        let t1774 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk538(t1212, t1717);
        let t1775 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk539(t1211, t1774);
        let (t1778, t1781, t1782, t1785) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk540(t1480, t344, t1225, t1469, t1012, t1770, t225);
    (t1761, t1763, t1765, t1767, t1769, t1770, t1774, t1775, t1778, t1781, t1782, t1785)
}
