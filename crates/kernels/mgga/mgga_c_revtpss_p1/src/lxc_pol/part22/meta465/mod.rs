//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta465 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2146;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta465<F: Float>(t1079: F, t15578: F, t3215: F, t4858: F, t372: F, t4872: F, t4786: F, t4873: F, t11696: F, t4781: F, t3092: F, t11705: F) -> (F, F, F, F, F, F, F, F) {
        let (t15579, t15583, t15584, t15585, t15586, t15591, t15592, t15595) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2146::<F>(t1079, t15578, t3215, t4858, t372, t4872, t4786, t4873, t11696, t4781, t3092, t11705);
    (t15579, t15583, t15584, t15585, t15586, t15591, t15592, t15595)
}
