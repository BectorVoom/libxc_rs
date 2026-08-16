//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta86 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk542;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk543;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk544;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk545;
use chunk4::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk546;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta86<F: Float>(t1721: F, t1735: F, t1761: F, t1763: F, t1767: F, t482: F, t1250: F, t1042: F, t476: F, t51: F, t52: F, t475: F, t467: F, t1264: F, t1715: F, t247: F, t1221: F, t1222: F, t1235: F, t1247: F, t1258: F, t1261: F, t1778: F, t1782: F, t1786: F, t1791: F, t464: F, t484: F) -> (F, F, F, F, F, F, F, F) {
        let t1794 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk542::<F>(t1721, t1735, t1761, t1763, t1767);
        let (t1796, t1797) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk543::<F>(t1794, t482, t1250, t1042);
        let (t1802, t1803) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk544::<F>(t476, t51, t52, t475);
        let (t1804, t1808) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk545::<F>(t1803, t467, t1264, t1715, t247);
        let t1811 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk546::<F>(t1221, t1222, t1235, t1247, t1258, t1261, t1778, t1782, t1786, t1791, t1797, t1804, t1808, t464, t484);
    (t1794, t1796, t1797, t1802, t1803, t1804, t1808, t1811)
}
