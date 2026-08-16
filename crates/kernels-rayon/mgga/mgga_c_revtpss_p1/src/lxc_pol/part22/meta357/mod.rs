//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta357 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1863;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1864;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1865;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1866;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1867;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta357(t3316: f64, t989: f64, t11239: f64, t11627: f64, t342: f64, t1129: f64, t3431: f64, t408: f64, t3434: f64, t421: f64, t1130: f64, t3376: f64, t1126: f64, t3432: f64, t418: f64, t3418: f64, t698: f64, t240: f64, t3698: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12160, t12166) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1863(t3316, t989, t11239, t11627);
        let (t12167, t12226, t12227) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1864(t12166, t342, t1129, t3431, t408);
        let (t12230, t12238, t12243) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1865(t3434, t421, t1130, t3376, t1126, t3432);
        let (t12247, t12248) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1866(t3431, t418, t408);
        let (t12252, t12254) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1867(t3418, t698, t240, t3698);
    (t12160, t12166, t12167, t12226, t12227, t12230, t12238, t12243, t12247, t12248, t12252, t12254)
}
