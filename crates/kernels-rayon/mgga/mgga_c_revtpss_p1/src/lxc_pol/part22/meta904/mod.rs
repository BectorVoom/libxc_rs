//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta904 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3101;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3102;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta904(t1011: f64, t3252: f64, t4574: f64, t697: f64, t1062: f64, t15887: f64, t11921: f64, t15837: f64, t247: f64, t4837: f64, t11267: f64, t4878: f64, t11263: f64, t4879: f64, t11773: f64, t3278: f64, t11875: f64, t11922: f64, t15898: f64, t15728: f64, t15827: f64, t11672: f64, t15984: f64, t16052: f64, t16055: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t54126, t54137, t54142, t54144) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3101(t1011, t3252, t4574, t697, t1062, t15887, t11921, t15837, t247, t4837, t11267, t4878);
        let (t54147, t54166, t54187, t54198, t54222, t54259) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3102(t11263, t4879, t11773, t3278, t11875, t11922, t15898, t15728, t15827, t11672, t15984, t16052, t16055);
    (t54126, t54137, t54142, t54144, t54147, t54166, t54187, t54198, t54222, t54259)
}
