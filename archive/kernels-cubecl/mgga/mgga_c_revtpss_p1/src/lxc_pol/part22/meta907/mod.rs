//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta907 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3107;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3108;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta907<F: Float>(t12021: F, t4820: F, t11922: F, t15921: F, t3115: F, t1086: F, t15669: F, t3090: F, t43347: F, t53668: F, t16163: F, t3124: F, t11875: F, t15605: F, t11852: F, t41270: F, t15905: F, t43384: F, t15595: F, t3091: F, t43131: F, t11675: F, t15984: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t54490, t54497, t54500, t54509, t54521) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3107::<F>(t12021, t4820, t11922, t15921, t3115, t1086, t15669, t3090, t43347, t53668, t16163, t3124);
        let (t54533, t54537, t54542, t54546, t54550) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3108::<F>(t11875, t11922, t15605, t11852, t41270, t15905, t43384, t15595, t3091, t43131, t11675, t15984);
    (t54490, t54497, t54500, t54509, t54521, t54533, t54537, t54542, t54546, t54550)
}
