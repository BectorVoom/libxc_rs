//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta83 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk576;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk577;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk578;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk579;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk580;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta83<F: Float>(t1161: F, t1180: F, t1721: F, t1735: F, t1737: F, t1745: F, t1750: F, t1757: F, t300: F, t435: F, t1179: F, t1188: F, t1756: F, t1196: F, t1201: F, t1717: F, t459: F, t1212: F, t1211: F, t1480: F, t344: F, t1225: F, t1469: F, t1012: F, t225: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t1761, t1763, t1765) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk576::<F>(t1161, t1180, t1721, t1735, t1737, t1745, t1750, t1757, t300, t435, t1179, t1188, t1756);
        let (t1767, t1769, t1770) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk577::<F>(t1196, t1765, t1201, t1717, t459);
        let t1774 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk578::<F>(t1212, t1717);
        let t1775 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk579::<F>(t1211, t1774);
        let (t1778, t1781, t1782, t1785) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk580::<F>(t1480, t344, t1225, t1469, t1012, t1770, t225);
    (t1761, t1763, t1765, t1767, t1769, t1770, t1774, t1775, t1778, t1781, t1782, t1785)
}
