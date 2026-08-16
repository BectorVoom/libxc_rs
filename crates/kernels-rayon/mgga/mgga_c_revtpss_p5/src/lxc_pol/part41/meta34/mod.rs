//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta34 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk212;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk213;
use chunk2::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk214;
use chunk3::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk215;
use chunk4::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk216;
use chunk5::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk217;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta34(t114: f64, t655: f64, t665: f64, t653: f64, t69: f64, t508: f64, t3: f64, t65: f64, t125: f64, t123: f64, t147: f64, t143: f64, t130: f64, t131: f64, t72: f64, t122: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t666, t670) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk212(t114, t655, t665, t653, t69);
        let (t671, t675) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk213(t508, t670, t3, t65);
        let t676 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk214(t125, t675);
        let t679 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk215(t123, t147, t676);
        let (t680, t681, t682, t684, t685) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk216(t143, t130, t131, t72, t122, t125);
        let t686 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk217(t675, t685);
    (t666, t670, t671, t675, t676, t679, t680, t681, t682, t684, t685, t686)
}
