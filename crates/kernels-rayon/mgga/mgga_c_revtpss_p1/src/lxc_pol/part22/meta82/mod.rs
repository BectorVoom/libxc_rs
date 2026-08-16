//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta82 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk595;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk596;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk597;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk598;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk599;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta82(t265: f64, t393: f64, t1076: f64, t1647: f64, t1652: f64, t1680: f64, t1696: f64, t342: f64, t386: f64, t995: f64, t1102: f64, t1587: f64, t1598: f64, t1612: f64, t1638: f64, t1640: f64, t1644: f64, t198: f64, t336: f64, t30: f64, t1468: f64, t1469: f64, t395: f64, t45: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64, t1121: f64, t1120: f64, t128: f64, t1119: f64, t422: f64, t1118: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1699, t1704) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk595(t265, t393, t1076, t1647, t1652, t1680, t1696, t342, t386, t995, t1102, t1587, t1598, t1612, t1638, t1640, t1644, t198, t336);
        let (t1709, t1711) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk596(t30, t1468, t1469, t1587, t1704, t265, t395, t45, dens_threshold, rho0, zeta_threshold);
        let t1715 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk597(t1121, t1469);
        let (t1716, t1717, t1719) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk598(t1120, t1715, t128, t1119);
        let (t1721, t1723) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk599(t1719, t422, t1118, t1717);
    (t1699, t1704, t1709, t1711, t1715, t1716, t1717, t1719, t1721, t1723)
}
