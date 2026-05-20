//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta368 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1690;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1691;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1692;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1693;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1694;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1695;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta368<F: Float>(t15670: F, t366: F, t3106: F, t4817: F, t11710: F, t4787: F, t3091: F, t245: F, t4890: F, t3088: F, t3317: F, t1065: F, t1668: F, t372: F, t4823: F, t1087: F, t11773: F, t4801: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t15671, t15675, t15682, t15684, t15687, t15688) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1690::<F>(t15670, t366, t3106, t4817, t11710, t4787, t3091, t245, t4890, t3088);
        let t15689 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1691::<F>(t15688, t3317);
        let (t15690, t15691) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1692::<F>(t1065, t1668, t372);
        let t15696 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1693::<F>(t372, t4823);
        let t15700 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1694::<F>(t1087, t11773);
        let t15701 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1695::<F>(t372, t4801);
    (t15671, t15675, t15682, t15684, t15687, t15688, t15689, t15690, t15691, t15696, t15700, t15701)
}
