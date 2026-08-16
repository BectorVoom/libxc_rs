//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta158 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk694;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk695;
use chunk2::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk696;
use chunk3::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk697;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta158(t4343: f64, t828: f64, t855: f64, t1544: f64, t221: f64, t2675: f64, t2674: f64, t1558: f64, t243: f64, t231: f64, t2662: f64, t2661: f64, t1565: f64, t2652: f64, t1561: f64, t2741: f64, t241: f64, t2719: f64, t820: f64, t72: f64, t245: f64, t125: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4345, t4349, t4350, t4352, t4353, t4354, t4355) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk694(t4343, t828, t855, t1544, t221, t2675, t2674, t1558, t243, t231, t2662, t2661);
        let (t4357, t4359, t4362) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk695(t1565, t2652, t1561, t2741, t241, t2719, t820);
        let t4364 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk696(t243, t72, t245);
        let t4365 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk697(t125, t1558);
    (t4345, t4349, t4350, t4352, t4353, t4354, t4355, t4357, t4359, t4362, t4364, t4365)
}
