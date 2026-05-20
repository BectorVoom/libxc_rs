//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta479 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2187;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2188;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2189;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2190;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta479<F: Float>(t3151: F, t3154: F, t15907: F, t3117: F, t11795: F, t11859: F, t11866: F, t11875: F, t15859: F, t15862: F, t15865: F, t15866: F, t15888: F, t15892: F, t15895: F, t15899: F, t15906: F, t3184: F, t375: F, t4834: F, t4912: F, t12160: F, t4891: F, t1043: F, t4772: F, t1045: F, t1086: F, t4746: F, t3090: F, t15822: F, t3160: F, t1065: F, t2852: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t15908, t15909, t15910, t15913) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2187::<F>(t3151, t3154, t15907, t3117, t11795, t11859, t11866, t11875, t15859, t15862, t15865, t15866, t15888, t15892, t15895, t15899, t15906, t3184, t375, t4834, t4912);
        let t15917 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2188::<F>(t12160, t4891);
        let (t15920, t15921, t15922, t15925, t15926) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2189::<F>(t1043, t4772, t1045, t3117, t1086, t4746, t3090);
        let (t15932, t15935) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2190::<F>(t15822, t3160, t1065, t2852);
    (t15908, t15909, t15910, t15913, t15917, t15920, t15921, t15922, t15925, t15926, t15932, t15935)
}
