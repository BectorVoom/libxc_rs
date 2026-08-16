//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta42 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk258;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk259;
use chunk2::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk260;
use chunk3::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk261;
use chunk4::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk262;
use chunk5::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk263;
use chunk6::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk264;
use chunk7::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk265;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta42(t72: f64, t826: f64, t125: f64, t245: f64, t225: f64, t679: f64, t704: f64, t709: f64, t718: f64, t751: f64, t754: f64, t759: f64, t764: f64, t243: f64, t73: f64, t775: f64, t227: f64, t229: f64, t231: f64, t587: f64, t66: f64, t240: f64, t247: f64, t237: f64, t233: f64, t235: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t827 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk258(t72, t826);
        let t828 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk259(t125, t245);
        let (t830, t832) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk260(t225, t679, t704, t709, t718, t751, t754, t759, t764, t243, t73);
        let (t833, t836) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk261(t775, t832, t227, t229, t830);
        let t837 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk262(t231, t836);
        let (t839, t843) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk263(t828, t837, t827, t587, t66);
        let t844 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk264(t240, t843);
        let (t848, t849) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk265(t243, t844, t247, t237, t233, t235);
    (t827, t828, t830, t832, t833, t836, t837, t839, t843, t844, t848, t849)
}
