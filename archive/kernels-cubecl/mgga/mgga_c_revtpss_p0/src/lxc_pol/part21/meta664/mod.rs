//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta664 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2460;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2461;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta664<F: Float>(t11671: F, t3278: F, t12020: F, t3168: F, t2434: F, t246: F, t1041: F, t1046: F, t11256: F, t11258: F, t3172: F, t11727: F, t3188: F, t12004: F, t3111: F, t1011: F, t11165: F, t15987: F, t11156: F, t15993: F, t11692: F, t11922: F, t4899: F, t1086: F, t11213: F, t3090: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t42967, t42970, t42994, t42996, t43003, t43017) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2460::<F>(t11671, t3278, t12020, t3168, t2434, t246, t1041, t1046, t11256, t11258, t3172, t11727, t3188);
        let (t43019, t43029, t43032, t43035, t43038) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2461::<F>(t12004, t3111, t1011, t11165, t15987, t11156, t15993, t11692, t11922, t4899, t1086, t11213, t3090);
    (t42967, t42970, t42994, t42996, t43003, t43017, t43019, t43029, t43032, t43035, t43038)
}
