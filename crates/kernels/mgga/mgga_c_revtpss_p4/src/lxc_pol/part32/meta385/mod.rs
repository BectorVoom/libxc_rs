//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta385 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1345;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1346;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta385<F: Float>(t17350: F, t3782: F, t1263: F, t1794: F, t372: F, t11262: F, t1796: F, t1247: F, t12915: F, t247: F, t5230: F, t5384: F, t12772: F, t5406: F, t3625: F, t1802: F, t474: F, t3089: F, t3717: F, t1284: F, t5219: F, t3624: F, t1230: F, t5390: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t17351, t17353, t17362, t17375) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1345::<F>(t17350, t3782, t1263, t1794, t372, t11262, t1796, t1247, t12915, t247, t5230, t5384);
        let (t17386, t17394, t17395, t17396, t17401, t17412) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1346::<F>(t12772, t5406, t3625, t1802, t474, t3089, t3717, t1284, t5219, t3624, t1230, t5390);
    (t17351, t17353, t17362, t17375, t17386, t17394, t17395, t17396, t17401, t17412)
}
