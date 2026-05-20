//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta914 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3121;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3122;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta914<F: Float>(t11792: F, t4845: F, t15749: F, t3224: F, t11922: F, t16039: F, t3115: F, t11859: F, t15610: F, t1032: F, t1040: F, t15886: F, t15690: F, t3153: F, t372: F, t11921: F, t15716: F, t15717: F, t247: F, t1041: F, t1670: F, t42994: F, t15786: F, t4892: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t55152, t55154, t55171, t55182, t55195) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3121::<F>(t11792, t4845, t15749, t3224, t11922, t16039, t3115, t11859, t15610, t1032, t1040, t15886);
        let (t55209, t55233, t55247, t55265) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3122::<F>(t15690, t3153, t372, t11921, t15716, t15717, t247, t1041, t1670, t42994, t11922, t15786, t4892);
    (t55152, t55154, t55171, t55182, t55195, t55209, t55233, t55247, t55265)
}
