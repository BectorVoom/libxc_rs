//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta501 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2111;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2112;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2113;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2114;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta501<F: Float>(t13396: F, t4806: F, t1042: F, t1651: F, t3075: F, t247: F, t3116: F, t1066: F, t15193: F, t1062: F, t4797: F, t1047: F, t1063: F, t1068: F, t11991: F, t15817: F, t15823: F, t15829: F, t15830: F, t1675: F, t3136: F, t3157: F, t3177: F, t3188: F, t4831: F, t4834: F, t4837: F, t4879: F, t1659: F, t3230: F, t1660: F, t3201: F, t1058: F, t4798: F, t1053: F, t15127: F, t15125: F, t15191: F, t11134: F, t11136: F, t11138: F, t11140: F, t11890: F, t15132: F, t15137: F, t15142: F, t15147: F, t15151: F, t15156: F, t15160: F, t15189: F, t15195: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t15833, t15834, t15837) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2111::<F>(t13396, t4806, t1042, t1651, t3075);
        let (t15839, t15847, t15850) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2112::<F>(t15837, t247, t3116, t1066, t15193, t1062, t4797);
        let t15855 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2113::<F>(t1047, t1063, t1068, t11991, t15817, t15823, t15829, t15830, t15834, t15839, t15847, t15850, t1675, t3136, t3157, t3177, t3188, t4831, t4834, t4837, t4879);
        let (t15859, t15862, t15865, t15866, t15885) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2114::<F>(t1659, t3230, t1660, t3201, t1058, t4798, t1053, t4797, t15127, t15125, t15191, t11134, t11136, t11138, t11140, t11890, t15132, t15137, t15142, t15147, t15151, t15156, t15160, t15189, t15195);
    (t15833, t15834, t15837, t15839, t15847, t15850, t15855, t15859, t15862, t15865, t15866, t15885)
}
