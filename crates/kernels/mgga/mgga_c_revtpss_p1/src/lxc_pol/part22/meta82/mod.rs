//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta82 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk595;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk596;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk597;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk598;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk599;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta82<F: Float>(t265: F, t393: F, t1076: F, t1647: F, t1652: F, t1680: F, t1696: F, t342: F, t386: F, t995: F, t1102: F, t1587: F, t1598: F, t1612: F, t1638: F, t1640: F, t1644: F, t198: F, t336: F, t30: F, t1468: F, t1469: F, t395: F, t45: F, dens_threshold: F, rho0: F, zeta_threshold: F, t1121: F, t1120: F, t128: F, t1119: F, t422: F, t1118: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t1699, t1704) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk595::<F>(t265, t393, t1076, t1647, t1652, t1680, t1696, t342, t386, t995, t1102, t1587, t1598, t1612, t1638, t1640, t1644, t198, t336);
        let (t1709, t1711) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk596::<F>(t30, t1468, t1469, t1587, t1704, t265, t395, t45, dens_threshold, rho0, zeta_threshold);
        let t1715 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk597::<F>(t1121, t1469);
        let (t1716, t1717, t1719) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk598::<F>(t1120, t1715, t128, t1119);
        let (t1721, t1723) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk599::<F>(t1719, t422, t1118, t1717);
    (t1699, t1704, t1709, t1711, t1715, t1716, t1717, t1719, t1721, t1723)
}
