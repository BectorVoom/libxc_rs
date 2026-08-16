//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta394 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1749;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1750;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1751;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta394<F: Float>(t17395: F, t3717: F, t1284: F, t5219: F, t3624: F, t1230: F, t5390: F, t12879: F, t1715: F, t247: F, t1261: F, t12916: F, t5342: F, t5340: F, t12702: F, t5330: F, t12744: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let t17396 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1749::<F>(t17395, t3717);
        let (t17400, t17401) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1750::<F>(t1284, t5219, t3624);
        let (t17412, t17416, t17417, t17423, t17425, t17426, t17429) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1751::<F>(t1230, t5390, t12879, t1715, t247, t1261, t12916, t5342, t5340, t12702, t5330, t12744);
    (t17396, t17400, t17401, t17412, t17416, t17417, t17423, t17425, t17426, t17429)
}
