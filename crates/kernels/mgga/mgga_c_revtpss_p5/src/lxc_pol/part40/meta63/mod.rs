//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta63 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk380;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk381;
use chunk2::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk382;
use chunk3::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk383;
use chunk4::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk384;
use chunk5::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk385;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta63<F: Float>(t225: F, t494: F, t1118: F, t1124: F, t139: F, t221: F, t462: F, t461: F, t1010: F, t56: F, t403: F, t404: F, t1121: F) -> (F, F, F, F, F, F, F, F, F) {
        let t1211 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk380::<F>(t225, t494);
        let (t1212, t1214) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk381::<F>(t1118, t1124);
        let t1215 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk382::<F>(t1211, t1214);
        let (t1219, t1221, t1222) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk383::<F>(t139, t221, t462, t461, t1010, t56);
        let t1224 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk384::<F>(t403, t404);
        let t1225 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk385::<F>(t1121, t1224);
    (t1211, t1212, t1214, t1215, t1219, t1221, t1222, t1224, t1225)
}
