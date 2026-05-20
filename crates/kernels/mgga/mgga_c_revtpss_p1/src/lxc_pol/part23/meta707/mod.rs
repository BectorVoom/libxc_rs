//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta707 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2460;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2461;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta707<F: Float>(t10174: F, t2453: F, t1420: F, t4075: F, t786: F, t1359: F, t39501: F, t10115: F, t555: F, t1445: F, t10165: F, t9664: F, t1427: F, t1444: F, t22: F, t9647: F, t123: F, t125: F, t1358: F, t8779: F, t9645: F, t268: F, t39644: F, t556: F, t561: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t47520, t47530, t47561, t47567, t47568, t47570) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2460::<F>(t10174, t2453, t1420, t4075, t786, t1359, t39501, t10115, t555, t1445, t10165, t9664);
        let (t47574, t47591, t47601) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2461::<F>(t1427, t1444, t22, t9647, t123, t125, t1358, t555, t8779, t9645, t268, t39644, t556, t561);
    (t47520, t47530, t47561, t47567, t47568, t47570, t47574, t47591, t47601)
}
