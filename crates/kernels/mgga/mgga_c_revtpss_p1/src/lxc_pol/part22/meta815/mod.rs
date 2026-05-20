//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta815 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2922;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2923;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2924;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta815<F: Float>(t1445: F, t47567: F, t10165: F, t9664: F, t1427: F, t1444: F, t22: F, t9647: F, t123: F, t2434: F, t4077: F, t9680: F, t125: F, t1358: F, t555: F, t8779: F, t9645: F, t2435: F, t9667: F, t268: F, t39644: F, t556: F, t561: F, t786: F, t9656: F, t10150: F, t2439: F, t4066: F, t785: F, t9303: F, t9641: F, t9635: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t47568, t47570, t47574, t47580) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2922::<F>(t1445, t47567, t10165, t9664, t1427, t1444, t22, t9647, t123, t2434, t4077, t9680);
        let (t47591, t47595, t47601) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2923::<F>(t123, t125, t1358, t555, t8779, t9645, t2435, t9667, t268, t39644, t556, t561);
        let (t47603, t47608, t47616, t47618, t47620) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2924::<F>(t556, t786, t9656, t10150, t2435, t1358, t2439, t4066, t785, t9303, t9641, t9635);
    (t47568, t47570, t47574, t47580, t47591, t47595, t47601, t47603, t47608, t47616, t47618, t47620)
}
