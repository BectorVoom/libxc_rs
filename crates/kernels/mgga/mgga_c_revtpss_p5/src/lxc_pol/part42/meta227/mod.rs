//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta227 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk882;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk883;
use chunk2::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk884;
use chunk3::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk885;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta227<F: Float>(t6096: F, t904: F, t128: F, t5825: F, t905: F, t2847: F, t4571: F, t6094: F, t291: F, t1610: F, t4590: F, t1609: F, t935: F, t2874: F, t1600: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t6097, t6098) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk882::<F>(t6096, t904, t128);
        let t6100 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk883::<F>(t5825, t905);
        let (t6101, t6102) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk884::<F>(t6100, t904, t128);
        let (t6104, t6106, t6108, t6109, t6110, t6112, t6113) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk885::<F>(t2847, t4571, t6094, t6098, t6102, t291, t1610, t4590, t1609, t935, t2874, t1600);
    (t6097, t6098, t6100, t6101, t6102, t6104, t6106, t6108, t6109, t6110, t6112, t6113)
}
