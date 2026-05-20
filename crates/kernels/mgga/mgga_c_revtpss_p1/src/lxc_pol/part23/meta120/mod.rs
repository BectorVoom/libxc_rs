//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta120 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk779;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk780;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk781;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk782;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk783;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta120<F: Float>(t1021: F, t1058: F, t371: F, t373: F, t676: F, t367: F, t225: F, t3057: F, t366: F, t1024: F, t1053: F, t1026: F, t127: F, t1025: F, t3046: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t3194, t3201) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk779::<F>(t1021, t1058, t371, t373, t676);
        let (t3203, t3204) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk780::<F>(t3201, t367, t225, t3057);
        let (t3205, t3211) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk781::<F>(t3204, t366, t1024, t1053);
        let (t3215, t3216, t3223) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk782::<F>(t1026, t127, t371, t1025, t225, t3046);
        let t3224 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk783::<F>(t3223, t366);
    (t3194, t3201, t3203, t3204, t3205, t3211, t3215, t3216, t3223, t3224)
}
