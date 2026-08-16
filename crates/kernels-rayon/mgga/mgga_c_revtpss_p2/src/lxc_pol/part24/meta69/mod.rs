//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta69 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk431;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk432;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk433;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk434;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk435;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta69(t1196: f64, t1765: f64, t1201: f64, t1717: f64, t459: f64, t1212: f64, t1211: f64, t1480: f64, t344: f64, t1225: f64, t1469: f64, t1012: f64, t225: f64, t480: f64, t482: f64, t372: f64, t371: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1767, t1769, t1770) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk431(t1196, t1765, t1201, t1717, t459);
        let t1774 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk432(t1212, t1717);
        let t1775 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk433(t1211, t1774);
        let (t1778, t1781, t1782, t1785) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk434(t1480, t344, t1225, t1469, t1012, t1770, t225);
        let (t1786, t1789, t1791) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk435(t1785, t480, t1774, t482, t372, t371);
    (t1767, t1769, t1770, t1774, t1775, t1778, t1781, t1782, t1785, t1786, t1789, t1791)
}
