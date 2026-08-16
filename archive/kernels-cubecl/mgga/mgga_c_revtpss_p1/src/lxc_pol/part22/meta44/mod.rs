//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta44 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk326;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk327;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk328;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk329;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk330;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk331;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta44<F: Float>(t233: F, t251: F, t869: F, t689: F, t234: F, t786: F, t72: F, t686: F, t822: F, t837: F, t860: F, t213: F, t820: F, t868: F, t783: F, t791: F, t862: F, t865: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t870, t871, t873, t874) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk326::<F>(t233, t251, t869, t689, t234, t786);
        let t875 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk327::<F>(t251, t72);
        let (t878, t879) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk328::<F>(t686, t874, t875, t251, t822);
        let (t880, t883, t886) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk329::<F>(t837, t879, t234, t860, t213, t820, t873, t878);
        let t887 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk330::<F>(t868, t886);
        let t890 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk331::<F>(t213, t783, t791, t862, t865, t887);
    (t870, t871, t873, t874, t875, t878, t879, t880, t883, t886, t887, t890)
}
