//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta43 (260520-c91 hierarchical CSE).
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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk262;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk263;
use chunk2::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk264;
use chunk3::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk265;
use chunk4::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk266;
use chunk5::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk267;
use chunk6::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk268;
use chunk7::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk269;
use chunk8::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk270;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta43<F: Float>(t72: F, t826: F, t125: F, t245: F, t225: F, t679: F, t704: F, t709: F, t718: F, t751: F, t754: F, t759: F, t764: F, t243: F, t73: F, t775: F, t227: F, t229: F, t231: F, t587: F, t66: F, t240: F, t247: F, t237: F, t233: F, t235: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t827 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk262::<F>(t72, t826);
        let t828 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk263::<F>(t125, t245);
        let t830 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk264::<F>(t225, t679, t704, t709, t718, t751, t754, t759, t764);
        let t832 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk265::<F>(t243, t73);
        let (t833, t836) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk266::<F>(t775, t832, t227, t229, t830);
        let t837 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk267::<F>(t231, t836);
        let (t839, t843) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk268::<F>(t828, t837, t827, t587, t66);
        let t844 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk269::<F>(t240, t843);
        let (t848, t849) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk270::<F>(t243, t844, t247, t237, t233, t235);
    (t827, t828, t830, t832, t833, t836, t837, t839, t843, t844, t848, t849)
}
