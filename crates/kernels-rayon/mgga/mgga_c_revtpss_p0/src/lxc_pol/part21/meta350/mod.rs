//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta350 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1688;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1689;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1690;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1691;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta350(t11696: f64, t3093: f64, t3092: f64, t11644: f64, t11649: f64, t11653: f64, t11656: f64, t11663: f64, t11667: f64, t11672: f64, t11675: f64, t11680: f64, t11684: f64, t11689: f64, t11693: f64, t3091: f64, t3097: f64, t3130: f64, t3136: f64, t3169: f64, t4837: f64, t4892: f64, t4899: f64, t3182: f64, t828: f64, t2852: f64, t357: f64, t2251: f64, t3109: f64, t3096: f64, t1020: f64, t3105: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11697, t11698, t11701) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1688(t11696, t3093, t3092, t11644, t11649, t11653, t11656, t11663, t11667, t11672, t11675, t11680, t11684, t11689, t11693, t3091, t3097, t3130, t3136, t3169, t4837, t4892, t4899);
        let t11703 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1689(t3182, t828);
        let (t11704, t11705, t11706, t11707, t11710) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1690(t2852, t357, t2251, t3093, t11703, t3109, t828);
        let (t11711, t11712, t11714) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1691(t11710, t3096, t3091, t1020, t3105);
    (t11697, t11698, t11701, t11703, t11704, t11705, t11706, t11707, t11710, t11711, t11712, t11714)
}
