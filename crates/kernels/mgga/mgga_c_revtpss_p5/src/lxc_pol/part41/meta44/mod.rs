//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta44 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk273;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk274;
use chunk2::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk275;
use chunk3::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk276;
use chunk4::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk277;
use chunk5::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk278;
use chunk6::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk279;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta44<F: Float>(t212: F, t225: F, t233: F, t251: F, t689: F, t234: F, t786: F, t72: F, t686: F, t822: F, t837: F, t860: F, t213: F, t820: F, t868: F, t783: F, t791: F, t862: F, t865: F, t261: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t869 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk273::<F>(t212, t225);
        let (t870, t871, t873, t874) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk274::<F>(t233, t251, t869, t689, t234, t786);
        let (t875, t878, t879) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk275::<F>(t251, t72, t686, t874, t822);
        let t886 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk276::<F>(t837, t879, t234, t860, t213, t820, t873, t878);
        let t887 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk277::<F>(t868, t886);
        let t890 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk278::<F>(t213, t783, t791, t862, t865, t887);
        let t892 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk279::<F>(t261);
    (t869, t870, t871, t873, t874, t875, t878, t879, t886, t887, t890, t892)
}
