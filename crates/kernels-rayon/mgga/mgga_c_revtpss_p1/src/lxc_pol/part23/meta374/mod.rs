//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta374 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1705;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1706;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta374(t11249: f64, t1668: f64, t12160: f64, t4891: f64, t1086: f64, t4746: f64, t3090: f64, t15822: f64, t3160: f64, t1065: f64, t2852: f64, t3173: f64, t4879: f64, t4866: f64, t73: f64, t11710: f64, t4782: f64, t3091: f64, t1014: f64, t140: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15907, t15917, t15925, t15926) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1705(t11249, t1668, t12160, t4891, t1086, t4746, t3090);
        let (t15932, t15935, t15942, t15957, t15984, t15986, t15987) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1706(t15822, t3160, t1065, t2852, t3173, t4879, t4866, t73, t11710, t4782, t3091, t1014, t140);
    (t15907, t15917, t15925, t15926, t15932, t15935, t15942, t15957, t15984, t15986, t15987)
}
