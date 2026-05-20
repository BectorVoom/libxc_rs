//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta457 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2130;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta457<F: Float>(t11528: F, t4595: F, t11294: F, t4636: F, t4632: F, t934: F, t2874: F, t1610: F, t2918: F, t2875: F, t4635: F, t11299: F) -> (F, F, F, F, F, F, F, F) {
        let (t15377, t15379, t15380, t15382, t15383, t15385, t15386, t15388) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2130::<F>(t11528, t4595, t11294, t4636, t4632, t934, t2874, t1610, t2918, t2875, t4635, t11299);
    (t15377, t15379, t15380, t15382, t15383, t15385, t15386, t15388)
}
