//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta51 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk310;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk311;
use chunk2::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk312;
use chunk3::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk313;
use chunk4::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk314;
use chunk5::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk315;
use chunk6::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk316;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta51<F: Float>(t225: F, t385: F, t902: F, t908: F, t344: F, t614: F, t139: F, t221: F, t346: F, t345: F, t220: F, t44: F, t124: F, t65: F, t270: F, t271: F, t905: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t996 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk310::<F>(t225, t385);
        let (t997, t999) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk311::<F>(t902, t908);
        let (t1000, t1003, t1007, t1009, t1010) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk312::<F>(t996, t999, t344, t614, t139, t221, t346, t345, t220);
        let t1011 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk313::<F>(t1010, t44);
        let t1012 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk314::<F>(t124, t65);
        let t1014 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk315::<F>(t270, t271);
        let t1015 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk316::<F>(t1014, t905);
    (t996, t997, t999, t1000, t1003, t1007, t1009, t1010, t1011, t1012, t1014, t1015)
}
