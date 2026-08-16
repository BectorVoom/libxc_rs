//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta33 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk245;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk246;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk247;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk248;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk249;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk250;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta33(t100: f64, t658: f64, t108: f64, t101: f64, t105: f64, t656: f64, t97: f64, t114: f64, t655: f64, t653: f64, t69: f64, t508: f64, t3: f64, t65: f64, t125: f64, t123: f64, t147: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t659, t661) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk245(t100, t658);
        let (t662, t665) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk246(t108, t661, t101, t105, t656, t659, t97);
        let (t666, t670) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk247(t114, t655, t665, t653, t69);
        let (t671, t675) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk248(t508, t670, t3, t65);
        let t676 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk249(t125, t675);
        let t679 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk250(t123, t147, t676);
    (t661, t662, t665, t666, t670, t671, t675, t676, t679)
}
