//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta258 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1588;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1589;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1590;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1591;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1592;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1593;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1594;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta258(t1131: f64, t6471: f64, t3435: f64, t6438: f64, t3433: f64, t3439: f64, t5044: f64, t6423: f64, t6427: f64, t6431: f64, t1744: f64, t1169: f64, t3459: f64, t3466: f64, t5093: f64, t6443: f64, t6450: f64, t6456: f64, t6458: f64, t6462: f64, t6465: f64, t6468: f64, t3479: f64, t3483: f64, t448: f64, t1756: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t6473, t6474) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1588(t1131, t6471, t3435, t6438);
        let (t6476, t6481, t6486, t6487) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1589(t3433, t6474, t3439, t5044, t6423, t6427, t6431, t1744, t1169);
        let t6502 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1590(t3459, t3466, t5044, t5093, t6423, t6427, t6431, t6443, t6450, t6456, t6458, t6462, t6465, t6468);
        let t6503 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1591(t1169, t6502);
        let t6506 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1592(t3479, t6486);
        let t6513 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1593(t3483, t5044, t6423, t6427, t6431);
        let (t6514, t6518) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1594(t448, t6513, t1756);
    (t6473, t6474, t6476, t6481, t6486, t6487, t6502, t6503, t6506, t6513, t6514, t6518)
}
