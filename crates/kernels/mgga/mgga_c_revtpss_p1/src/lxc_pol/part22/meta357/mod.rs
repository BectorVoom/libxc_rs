//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta357 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1863;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1864;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1865;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1866;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1867;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta357<F: Float>(t3316: F, t989: F, t11239: F, t11627: F, t342: F, t1129: F, t3431: F, t408: F, t3434: F, t421: F, t1130: F, t3376: F, t1126: F, t3432: F, t418: F, t3418: F, t698: F, t240: F, t3698: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t12160, t12166) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1863::<F>(t3316, t989, t11239, t11627);
        let (t12167, t12226, t12227) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1864::<F>(t12166, t342, t1129, t3431, t408);
        let (t12230, t12238, t12243) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1865::<F>(t3434, t421, t1130, t3376, t1126, t3432);
        let (t12247, t12248) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1866::<F>(t3431, t418, t408);
        let (t12252, t12254) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1867::<F>(t3418, t698, t240, t3698);
    (t12160, t12166, t12167, t12226, t12227, t12230, t12238, t12243, t12247, t12248, t12252, t12254)
}
