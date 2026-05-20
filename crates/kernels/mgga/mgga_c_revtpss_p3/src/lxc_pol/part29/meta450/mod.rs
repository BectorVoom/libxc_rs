//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta450 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1682;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1683;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1684;
use chunk3::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1685;
use chunk4::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1686;
use chunk5::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1687;
use chunk6::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1688;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta450<F: Float>(t25877: F, t25894: F, t1385: F, t2028: F, t25875: F, t1399: F, t676: F, t1955: F, t4066: F, t1032: F, t1419: F, t545: F, t9656: F, t4075: F, t7282: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t25895 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1682::<F>(t25877, t25894);
        let t25898 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1683::<F>(t1385, t2028);
        let t25899 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1684::<F>(t25875, t25898);
        let (t25900, t25904) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1685::<F>(t1399, t676, t25894, t25898);
        let (t25909, t25920, t25921) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1686::<F>(t1955, t4066, t1032, t1419);
        let t25924 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1687::<F>(t545, t9656);
        let (t25929, t25930) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1688::<F>(t4075, t7282, t1955);
    (t25895, t25898, t25899, t25900, t25904, t25909, t25920, t25921, t25924, t25929, t25930)
}
