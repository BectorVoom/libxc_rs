//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta33 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk249;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk250;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk251;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk252;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk253;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk254;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk255;
use chunk7::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk256;
use chunk8::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk257;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta33(t43: f64, tau0: f64, t605: f64, t100: f64, t108: f64, t101: f64, t105: f64, t97: f64, t114: f64, t655: f64, t653: f64, t69: f64, t508: f64, t3: f64, t65: f64, t125: f64, t123: f64, t147: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t656 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk249(t43, tau0);
        let t658 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk250(t605);
        let (t659, t661) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk251(t100, t658);
        let (t662, t665) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk252(t108, t661, t101, t105, t656, t659, t97);
        let (t666, t670) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk253(t114, t655, t665, t653, t69);
        let t671 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk254(t508, t670);
        let t675 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk255(t3, t65);
        let t676 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk256(t125, t675);
        let t679 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk257(t123, t147, t676);
    (t656, t658, t659, t661, t662, t665, t666, t670, t671, t675, t676, t679)
}
