//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta478 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2183;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2184;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2185;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2186;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta478(t15886: f64, t225: f64, t366: f64, t1058: f64, t4794: f64, t1651: f64, t3151: f64, t3155: f64, t3117: f64, t3162: f64, t11243: f64, t72: f64, t3088: f64, t12078: f64, t11249: f64, t1668: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15887, t15888, t15892, t15893, t15894, t15895, t15898, t15899, t15904) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2183(t15886, t225, t366, t1058, t4794, t1651, t3151, t3155, t3117, t3162, t11243, t72);
        let t15905 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2184(t15904, t3088);
        let t15906 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2185(t12078, t15905);
        let t15907 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2186(t11249, t1668);
    (t15887, t15888, t15892, t15893, t15894, t15895, t15898, t15899, t15904, t15905, t15906, t15907)
}
