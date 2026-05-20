//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta514 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2144;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2145;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta514<F: Float>(t16152: F, t247: F, t3116: F, t3109: F, t4583: F, t1063: F, t3172: F, t4868: F, t1041: F, t2862: F, t4823: F, t1042: F, t1651: F, t3181: F, t2853: F, t15100: F, t15103: F, t15377: F, t15379: F, t15382: F, t15385: F, t15388: F, t15392: F, t15395: F, t15519: F, t15522: F, t15524: F, t15528: F, t15530: F, t15536: F, t15540: F, t15545: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t16154, t16158, t16160, t16163, t16165, t16166, t16167) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2144::<F>(t16152, t247, t3116, t3109, t4583, t1063, t3172, t4868, t1041, t2862, t4823, t1042);
        let (t16170, t16171, t16172, t16179) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2145::<F>(t1651, t3181, t2853, t1042, t15100, t15103, t15377, t15379, t15382, t15385, t15388, t15392, t15395, t15519, t15522, t15524, t15528, t15530, t15536, t15540, t15545);
    (t16154, t16158, t16160, t16163, t16165, t16166, t16167, t16170, t16171, t16172, t16179)
}
