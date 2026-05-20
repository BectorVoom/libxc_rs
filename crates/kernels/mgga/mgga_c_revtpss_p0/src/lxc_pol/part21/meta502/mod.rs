//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta502 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2115;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2116;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2117;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2118;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta502<F: Float>(t15885: F, t341: F, t225: F, t366: F, t1058: F, t4794: F, t1651: F, t3151: F, t3155: F, t3117: F, t3162: F, t11243: F, t72: F, t3088: F, t12078: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t15886 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2115::<F>(t15885, t341);
        let (t15887, t15888, t15892, t15893) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2116::<F>(t15886, t225, t366, t1058, t4794, t1651, t3151);
        let (t15894, t15895, t15898, t15899, t15904, t15905) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2117::<F>(t15893, t3155, t3117, t3162, t11243, t72, t3088);
        let t15906 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2118::<F>(t12078, t15905);
    (t15886, t15887, t15888, t15892, t15893, t15894, t15895, t15898, t15899, t15904, t15905, t15906)
}
