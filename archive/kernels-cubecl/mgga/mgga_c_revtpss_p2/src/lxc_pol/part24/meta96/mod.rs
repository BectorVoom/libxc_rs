//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta96 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk552;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk553;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk554;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk555;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk556;
use chunk5::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk557;
use chunk6::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk558;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta96<F: Float>(t1087: F, t3090: F, t1066: F, t828: F, t357: F, t905: F, t1065: F, t126: F, t1086: F, t994: F, t373: F, t66: F, t1024: F, t1062: F, t1031: F, t196: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let t3091 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk552::<F>(t1087, t3090);
        let t3092 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk553::<F>(t1066, t828);
        let (t3094, t3109) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk554::<F>(t357, t905, t1065, t126);
        let (t3114, t3115) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk555::<F>(t1086, t994, t3090);
        let (t3116, t3117) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk556::<F>(t373, t66, t828);
        let t3127 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk557::<F>(t1024, t1062);
        let t3140 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk558::<F>(t1031, t196);
    (t3091, t3092, t3094, t3109, t3114, t3115, t3116, t3117, t3127, t3140)
}
