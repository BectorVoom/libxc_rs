//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta253 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk964;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk965;
use chunk2::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk966;
use chunk3::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk967;
use chunk4::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk968;
use chunk5::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk969;
use chunk6::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk970;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta253(t2175: f64, t625: f64, t2339: f64, t69: f64, t101: f64, t96: f64, t665: f64, t43: f64, t655: f64, t100: f64, t114: f64, t658: f64, t508: f64, t569: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t8257, t8258) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk964(t2175, t625, t2339, t69);
        let t8259 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk965(t101, t96);
        let (t8260, t8264, t8267) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk966(t665, t8259, t101, t43, t655, t69);
        let t8268 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk967(t100, t96);
        let (t8269, t8273) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk968(t114, t658, t8268, t69, t8257, t8258, t8260, t8264, t8267);
        let t8274 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk969(t508, t8273);
        let t8278 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk970(t569, t8273);
    (t8257, t8258, t8259, t8260, t8264, t8267, t8268, t8269, t8273, t8274, t8278)
}
