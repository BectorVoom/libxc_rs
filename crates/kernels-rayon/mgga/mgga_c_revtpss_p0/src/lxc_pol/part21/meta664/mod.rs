//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta664 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2460;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2461;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta664(t11671: f64, t3278: f64, t12020: f64, t3168: f64, t2434: f64, t246: f64, t1041: f64, t1046: f64, t11256: f64, t11258: f64, t3172: f64, t11727: f64, t3188: f64, t12004: f64, t3111: f64, t1011: f64, t11165: f64, t15987: f64, t11156: f64, t15993: f64, t11692: f64, t11922: f64, t4899: f64, t1086: f64, t11213: f64, t3090: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t42967, t42970, t42994, t42996, t43003, t43017) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2460(t11671, t3278, t12020, t3168, t2434, t246, t1041, t1046, t11256, t11258, t3172, t11727, t3188);
        let (t43019, t43029, t43032, t43035, t43038) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2461(t12004, t3111, t1011, t11165, t15987, t11156, t15993, t11692, t11922, t4899, t1086, t11213, t3090);
    (t42967, t42970, t42994, t42996, t43003, t43017, t43019, t43029, t43032, t43035, t43038)
}
