//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta62 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk395;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk396;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk397;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk398;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk399;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta62<F: Float>(t1570: F, t1580: F, t213: F, t783: F, t791: F, t865: F, t1524: F, t1533: F, t1536: F, t1544: F, t198: F, t207: F, t679: F, t704: F, t751: F, t759: F, t764: F, t765: F, t892: F, t1469: F, t905: F, t904: F, t128: F, t903: F, t291: F, t902: F) -> (F, F, F, F, F, F, F, F) {
        let t1583 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk395::<F>(t1570, t1580, t213, t783, t791, t865);
        let t1587 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk396::<F>(t1524, t1533, t1536, t1544, t1583, t198, t207, t679, t704, t751, t759, t764, t765, t892);
        let t1592 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk397::<F>(t1469, t905);
        let (t1593, t1594, t1596) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk398::<F>(t1592, t904, t128, t903);
        let (t1598, t1600) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk399::<F>(t1596, t291, t1594, t902);
    (t1583, t1587, t1592, t1593, t1594, t1596, t1598, t1600)
}
