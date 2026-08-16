//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta87 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk527;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk528;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk529;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta87<F: Float>(t1211: F, t1774: F, t1480: F, t344: F, t1225: F, t1469: F, t1012: F, t1770: F, t225: F, t480: F, t482: F, t372: F, t371: F, t1721: F, t1735: F, t1761: F, t1763: F, t1767: F, t1250: F, t1042: F, t476: F, t51: F, t52: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t1775, t1778, t1781, t1782, t1785, t1786, t1789, t1790) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk527::<F>(t1211, t1774, t1480, t344, t1225, t1469, t1012, t1770, t225, t480, t482, t372);
        let (t1791, t1794) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk528::<F>(t1790, t371, t1721, t1735, t1761, t1763, t1767);
        let (t1796, t1797, t1802) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk529::<F>(t1794, t482, t1250, t1042, t476, t51, t52);
    (t1775, t1778, t1781, t1782, t1785, t1786, t1789, t1791, t1794, t1796, t1797, t1802)
}
