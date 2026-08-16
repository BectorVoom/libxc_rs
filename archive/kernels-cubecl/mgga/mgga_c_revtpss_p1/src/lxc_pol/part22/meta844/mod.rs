//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta844 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2978;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2979;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta844<F: Float>(t10142: F, t14113: F, t49180: F, t10136: F, t14239: F, t10119: F, t4101: F, t5740: F, t9288: F, t1419: F, t5658: F, t2782: F, t4086: F, t543: F, t40270: F, t5737: F, t13920: F, t555: F, t10073: F, t14207: F, t47973: F, t10090: F, t13805: F, t1882: F, t2482: F, t686: F, t72: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t49189, t49198, t49200, t49203, t49208) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2978::<F>(t10142, t14113, t49180, t10136, t14239, t10119, t4101, t5740, t9288, t1419, t5658, t2782, t4086, t543);
        let (t49210, t49213, t49238, t49242, t49248) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2979::<F>(t40270, t5737, t13920, t555, t10073, t14207, t2782, t4086, t47973, t543, t10090, t13805, t1882, t2482, t686, t72);
    (t49189, t49198, t49200, t49203, t49208, t49210, t49213, t49238, t49242, t49248)
}
