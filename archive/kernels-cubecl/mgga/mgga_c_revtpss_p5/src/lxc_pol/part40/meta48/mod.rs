//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta48 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk296;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk297;
use chunk2::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk298;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta48<F: Float>(t918: F, t923: F, t240: F, t696: F, t281: F, t283: F, t346: F, t906: F, t141: F, t908: F, t919: F, t921: F, t290: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t924, t926, t928, t929, t930) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk296::<F>(t918, t923, t240, t696, t281, t283, t346);
        let (t931, t932, t934) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk297::<F>(t906, t930, t141, t908, t919, t921, t924, t929);
        let t935 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk298::<F>(t290);
    (t924, t926, t928, t929, t930, t931, t932, t934, t935)
}
