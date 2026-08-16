//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta13 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk87;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk88;
use chunk2::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk89;
use chunk3::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk90;
use chunk4::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk91;
use chunk5::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk92;
use chunk6::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk93;
use chunk7::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk94;
use chunk8::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk95;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta13(t207: f64, t73: f64, t227: f64, t225: f64, t64: f64, t213: f64, t21: f64, t66: f64, t159: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t228, t229) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk87(t207, t73);
        let t231 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk88(t227, t229);
        let (t232, t233) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk89(t231);
        let t234 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk90(t225, t233);
        let t235 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk91(t64);
        let t236 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk92(t234, t235);
        let (t237, t239) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk93(t213, t236, t21, t66);
        let t240 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk94(t159);
        let t241 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk95(t239, t240);
    (t228, t229, t231, t232, t233, t234, t235, t236, t237, t239, t240, t241)
}
