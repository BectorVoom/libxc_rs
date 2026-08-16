//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta33 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk207;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk208;
use chunk2::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk209;
use chunk3::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk210;
use chunk4::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk211;
use chunk5::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk212;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta33(t5: f64, t599: f64, t603: f64, t644: f64, t91: f64, t117: f64, t116: f64, t94: f64, t112: f64, t625: f64, t111: f64, t43: f64, t605: f64, tau0: f64, t100: f64, t108: f64, t101: f64, t105: f64, t97: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t648 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk207(t5, t599, t603, t644, t91);
        let (t649, t651) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk208(t117, t648, t116, t94);
        let (t653, t654, t655) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk209(t112, t625, t111);
        let (t656, t658) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk210(t43, t605, tau0);
        let (t659, t661) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk211(t100, t658);
        let (t662, t665) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk212(t108, t661, t101, t105, t656, t659, t97);
    (t648, t649, t651, t653, t654, t655, t656, t658, t661, t662, t665)
}
