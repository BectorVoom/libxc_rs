//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta374 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1705;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1706;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta374<F: Float>(t11249: F, t1668: F, t12160: F, t4891: F, t1086: F, t4746: F, t3090: F, t15822: F, t3160: F, t1065: F, t2852: F, t3173: F, t4879: F, t4866: F, t73: F, t11710: F, t4782: F, t3091: F, t1014: F, t140: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t15907, t15917, t15925, t15926) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1705::<F>(t11249, t1668, t12160, t4891, t1086, t4746, t3090);
        let (t15932, t15935, t15942, t15957, t15984, t15986, t15987) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1706::<F>(t15822, t3160, t1065, t2852, t3173, t4879, t4866, t73, t11710, t4782, t3091, t1014, t140);
    (t15907, t15917, t15925, t15926, t15932, t15935, t15942, t15957, t15984, t15986, t15987)
}
