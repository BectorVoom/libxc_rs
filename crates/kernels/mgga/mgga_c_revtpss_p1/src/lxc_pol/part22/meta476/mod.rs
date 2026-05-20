//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta476 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2178;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2179;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2180;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta476<F: Float>(t13396: F, t4806: F, t1042: F, t1651: F, t3075: F, t247: F, t3116: F, t1066: F, t15193: F, t1062: F, t4797: F, t1047: F, t1063: F, t1068: F, t11991: F, t15817: F, t15823: F, t15829: F, t15830: F, t1675: F, t3136: F, t3157: F, t3177: F, t3188: F, t4831: F, t4834: F, t4837: F, t4879: F) -> (F, F, F, F, F, F, F) {
        let (t15833, t15834, t15837) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2178::<F>(t13396, t4806, t1042, t1651, t3075);
        let (t15839, t15847, t15850) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2179::<F>(t15837, t247, t3116, t1066, t15193, t1062, t4797);
        let t15855 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2180::<F>(t1047, t1063, t1068, t11991, t15817, t15823, t15829, t15830, t15834, t15839, t15847, t15850, t1675, t3136, t3157, t3177, t3188, t4831, t4834, t4837, t4879);
    (t15833, t15834, t15837, t15839, t15847, t15850, t15855)
}
