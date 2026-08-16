//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta28 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk181;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk182;
use chunk2::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk183;
use chunk3::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk184;
use chunk4::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk185;
use chunk5::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk186;
use chunk6::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk187;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta28(t543: f64, t225: f64, t235: f64, t213: f64, t531: f64, t241: f64, t247: f64, t217: f64, t535: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t544, t545) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk181(t543);
        let t546 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk182(t225, t545);
        let t547 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk183(t235, t546);
        let (t548, t549, t550) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk184(t213, t547, t531);
        let t555 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk185(t241, t550, t247, t217, t535, t548);
        let t556 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk186(t225, t555);
        let (t557, t560, t561) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk187(t546, t555, t213);
    (t544, t545, t546, t547, t548, t549, t550, t555, t556, t557, t560, t561)
}
