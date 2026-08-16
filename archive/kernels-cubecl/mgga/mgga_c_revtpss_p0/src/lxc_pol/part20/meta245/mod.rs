//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta245 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1065;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1066;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1067;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1068;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1069;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1070;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta245<F: Float>(t11160: F, t904: F, t128: F, t2258: F, t2857: F, t606: F, t10326: F, t905: F, t11133: F, t11134: F, t11136: F, t11138: F, t11140: F, t11147: F, t11153: F, t11158: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t11161, t11162) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1065::<F>(t11160, t904, t128);
        let t11165 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1066::<F>(t2258, t2857, t606);
        let (t11166, t11167) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1067::<F>(t11165, t904, t128);
        let t11169 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1068::<F>(t10326, t905);
        let (t11170, t11171) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1069::<F>(t11169, t904, t128);
        let t11173 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1070::<F>(t11133, t11134, t11136, t11138, t11140, t11147, t11153, t11158, t11162, t11167, t11171);
    (t11161, t11162, t11165, t11166, t11167, t11169, t11170, t11171, t11173)
}
