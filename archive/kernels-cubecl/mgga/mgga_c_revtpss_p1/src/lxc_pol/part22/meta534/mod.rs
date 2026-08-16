//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta534 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2334;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2335;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2336;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta534<F: Float>(t1284: F, t5219: F, t3624: F, t1225: F, t13312: F, t1012: F, t1230: F, t5390: F, t12879: F, t1715: F, t247: F, t1261: F, t16756: F, t5341: F, t3720: F, t12916: F, t5342: F, t5340: F, t12702: F, t5330: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t17400, t17401) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2334::<F>(t1284, t5219, t3624);
        let (t17404, t17405, t17412) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2335::<F>(t1225, t13312, t1012, t1230, t5390);
        let (t17416, t17417, t17419, t17420, t17423, t17425, t17426) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2336::<F>(t12879, t1715, t247, t1261, t16756, t5341, t3720, t12916, t5342, t5340, t12702, t5330);
    (t17400, t17401, t17404, t17405, t17412, t17416, t17417, t17419, t17420, t17423, t17425, t17426)
}
