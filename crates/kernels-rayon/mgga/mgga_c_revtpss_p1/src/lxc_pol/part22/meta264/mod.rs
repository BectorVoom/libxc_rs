//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta264 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1618;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1619;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1620;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1621;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta264(t225: f64, t494: f64, t6695: f64, t1828: f64, t3737: f64, t1280: f64, t6573: f64, t1287: f64, t6688: f64, t1774: f64, t5486: f64, t6587: f64, t487: f64, t6628: f64, t3769: f64, t1794: f64, t1811: f64, t6622: f64, t3783: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t6697, t6702) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1618(t225, t494, t6695, t1828);
        let t6703 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1619(t3737, t6702);
        let (t6714, t6717) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1620(t1280, t6573, t1287, t6688);
        let (t6720, t6723, t6727, t6731, t6735, t6738) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1621(t1774, t5486, t1280, t6587, t487, t6628, t3769, t1287, t1794, t1811, t6622, t3783);
    (t6697, t6702, t6703, t6714, t6717, t6720, t6723, t6727, t6731, t6735, t6738)
}
