//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta904 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3101;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3102;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta904<F: Float>(t1011: F, t3252: F, t4574: F, t697: F, t1062: F, t15887: F, t11921: F, t15837: F, t247: F, t4837: F, t11267: F, t4878: F, t11263: F, t4879: F, t11773: F, t3278: F, t11875: F, t11922: F, t15898: F, t15728: F, t15827: F, t11672: F, t15984: F, t16052: F, t16055: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t54126, t54137, t54142, t54144) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3101::<F>(t1011, t3252, t4574, t697, t1062, t15887, t11921, t15837, t247, t4837, t11267, t4878);
        let (t54147, t54166, t54187, t54198, t54222, t54259) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3102::<F>(t11263, t4879, t11773, t3278, t11875, t11922, t15898, t15728, t15827, t11672, t15984, t16052, t16055);
    (t54126, t54137, t54142, t54144, t54147, t54166, t54187, t54198, t54222, t54259)
}
