//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta290 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1157;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1158;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta290<F: Float>(t1089: F, t11928: F, t1071: F, t1086: F, t994: F, t11869: F, t3316: F, t989: F, t1082: F, t11804: F, t11239: F, t11627: F, t342: F, t11631: F, t12051: F, t12048: F, t1024: F, t1083: F, t1087: F, t11782: F, t12111: F, t12116: F, t12119: F, t12122: F, t12124: F, t12127: F, t12128: F, t12133: F, t12137: F, t12143: F, t12146: F, t12149: F, t3204: F, t3223: F, t3287: F, t3288: F, t3292: F, t3295: F, t3305: F, t3319: F, t4981: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t12150, t12153, t12154, t12157, t12160, t12163, t12166) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1157::<F>(t1089, t11928, t1071, t1086, t994, t11869, t3316, t989, t1082, t11804, t11239, t11627);
        let (t12167, t12168, t12169, t12172) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1158::<F>(t12166, t342, t11631, t12051, t12048, t1024, t1083, t1087, t11782, t12111, t12116, t12119, t12122, t12124, t12127, t12128, t12133, t12137, t12143, t12146, t12149, t12150, t12154, t12157, t12160, t12163, t3204, t3223, t3287, t3288, t3292, t3295, t3305, t3319, t4981);
    (t12150, t12153, t12154, t12157, t12160, t12163, t12166, t12167, t12168, t12169, t12172)
}
