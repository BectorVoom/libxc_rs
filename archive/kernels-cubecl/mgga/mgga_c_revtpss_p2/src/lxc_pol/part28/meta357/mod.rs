//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta357 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1378;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1379;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1380;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1381;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta357<F: Float>(t3431: F, t418: F, t408: F, t3418: F, t698: F, t240: F, t3698: F, t3361: F, t635: F, t1146: F, t2439: F, t3424: F, t3421: F, t57: F, t268: F, t404: F, t7021: F, t1123: F, t2435: F, t3364: F, t689: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t12248, t12252, t12254, t12256, t12261, t12263) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1378::<F>(t3431, t418, t408, t3418, t698, t240, t3698, t3361, t635, t1146, t2439, t3424);
        let (t12265, t12268, t12295) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1379::<F>(t3421, t698, t3361, t57, t268, t404, t7021);
        let (t12296, t12297) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1380::<F>(t12295, t1123, t2435);
        let t12299 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1381::<F>(t3364, t689);
    (t12248, t12252, t12254, t12256, t12261, t12263, t12265, t12268, t12295, t12296, t12297, t12299)
}
