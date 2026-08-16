//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta485 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2203;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2204;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta485(t11922: f64, t4895: f64, t4892: f64, t140: f64, t4886: f64, t1011: f64, t3241: f64, t4924: f64, t12047: f64, t15905: f64, t3151: f64, t357: f64, t15907: f64, t3117: f64, t11883: f64, t11888: f64, t16037: f64, t16040: f64, t16045: f64, t16049: f64, t16052: f64, t1656: f64, t3115: f64, t4887: f64, t4896: f64, t4902: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16055, t16057, t16060, t16062, t16064, t16067) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2203(t11922, t4895, t4892, t140, t4886, t1011, t3241, t4924, t12047, t15905);
        let (t16068, t16069, t16070, t16073) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2204(t3151, t357, t15907, t3117, t11883, t11888, t16037, t16040, t16045, t16049, t16052, t16057, t16062, t16064, t16067, t1656, t3115, t3241, t4887, t4896, t4902);
    (t16055, t16057, t16060, t16062, t16064, t16067, t16068, t16069, t16070, t16073)
}
