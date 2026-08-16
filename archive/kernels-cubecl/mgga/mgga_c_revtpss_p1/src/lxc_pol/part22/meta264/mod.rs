//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta264 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1618;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1619;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1620;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1621;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta264<F: Float>(t225: F, t494: F, t6695: F, t1828: F, t3737: F, t1280: F, t6573: F, t1287: F, t6688: F, t1774: F, t5486: F, t6587: F, t487: F, t6628: F, t3769: F, t1794: F, t1811: F, t6622: F, t3783: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t6697, t6702) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1618::<F>(t225, t494, t6695, t1828);
        let t6703 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1619::<F>(t3737, t6702);
        let (t6714, t6717) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1620::<F>(t1280, t6573, t1287, t6688);
        let (t6720, t6723, t6727, t6731, t6735, t6738) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1621::<F>(t1774, t5486, t1280, t6587, t487, t6628, t3769, t1287, t1794, t1811, t6622, t3783);
    (t6697, t6702, t6703, t6714, t6717, t6720, t6723, t6727, t6731, t6735, t6738)
}
