//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta350 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1688;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1689;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1690;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1691;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta350<F: Float>(t11696: F, t3093: F, t3092: F, t11644: F, t11649: F, t11653: F, t11656: F, t11663: F, t11667: F, t11672: F, t11675: F, t11680: F, t11684: F, t11689: F, t11693: F, t3091: F, t3097: F, t3130: F, t3136: F, t3169: F, t4837: F, t4892: F, t4899: F, t3182: F, t828: F, t2852: F, t357: F, t2251: F, t3109: F, t3096: F, t1020: F, t3105: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t11697, t11698, t11701) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1688::<F>(t11696, t3093, t3092, t11644, t11649, t11653, t11656, t11663, t11667, t11672, t11675, t11680, t11684, t11689, t11693, t3091, t3097, t3130, t3136, t3169, t4837, t4892, t4899);
        let t11703 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1689::<F>(t3182, t828);
        let (t11704, t11705, t11706, t11707, t11710) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1690::<F>(t2852, t357, t2251, t3093, t11703, t3109, t828);
        let (t11711, t11712, t11714) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1691::<F>(t11710, t3096, t3091, t1020, t3105);
    (t11697, t11698, t11701, t11703, t11704, t11705, t11706, t11707, t11710, t11711, t11712, t11714)
}
