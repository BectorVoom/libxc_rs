//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta907 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3107;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3108;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta907(t12021: f64, t4820: f64, t11922: f64, t15921: f64, t3115: f64, t1086: f64, t15669: f64, t3090: f64, t43347: f64, t53668: f64, t16163: f64, t3124: f64, t11875: f64, t15605: f64, t11852: f64, t41270: f64, t15905: f64, t43384: f64, t15595: f64, t3091: f64, t43131: f64, t11675: f64, t15984: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t54490, t54497, t54500, t54509, t54521) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3107(t12021, t4820, t11922, t15921, t3115, t1086, t15669, t3090, t43347, t53668, t16163, t3124);
        let (t54533, t54537, t54542, t54546, t54550) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3108(t11875, t11922, t15605, t11852, t41270, t15905, t43384, t15595, t3091, t43131, t11675, t15984);
    (t54490, t54497, t54500, t54509, t54521, t54533, t54537, t54542, t54546, t54550)
}
