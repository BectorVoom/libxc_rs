//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta136 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk652;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk653;
use chunk2::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk654;
use chunk3::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk655;
use chunk4::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk656;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta136<F: Float>(t3075: F, t996: F, t221: F, t346: F, t696: F, t345: F, t2270: F, t344: F, t1003: F, t1007: F, t360: F, t365: F, t1038: F, t72: F, t1087: F, t1066: F, t828: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t3076, t3080, t3082, t3083, t3086, t3088) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk652::<F>(t3075, t996, t221, t346, t696, t345, t2270, t344, t1003, t1007, t360, t365);
        let t3089 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk653::<F>(t1038, t72);
        let t3090 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk654::<F>(t3088, t3089);
        let t3091 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk655::<F>(t1087, t3090);
        let t3092 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk656::<F>(t1066, t828);
    (t3076, t3080, t3082, t3083, t3086, t3088, t3089, t3090, t3091, t3092)
}
