//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta899 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3091;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3092;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta899<F: Float>(t15711: F, t3106: F, t15935: F, t372: F, t15904: F, t245: F, t3088: F, t12167: F, t1063: F, t1592: F, t247: F, t42778: F, t11922: F, t16044: F, t3115: F, t11994: F, t15769: F, t3298: F, t4746: F, t4891: F, t11744: F, t4834: F, t12009: F, t15823: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t53724, t53728, t53739, t53740, t53741, t53762) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3091::<F>(t15711, t3106, t15935, t372, t15904, t245, t3088, t12167, t1063, t1592, t247, t42778);
        let (t53771, t53790, t53800, t53805, t53810) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3092::<F>(t11922, t16044, t3115, t11994, t15769, t3298, t4746, t4891, t11744, t4834, t12009, t15823);
    (t53724, t53728, t53739, t53740, t53741, t53762, t53771, t53790, t53800, t53805, t53810)
}
