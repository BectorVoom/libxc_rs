//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta253 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1086;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta253<F: Float>(t11315: F, t923: F, t11156: F, t2908: F, t141: F, t11165: F, t930: F, t2912: F, t698: F, t11151: F, t11160: F, t11132: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t11316, t11318, t11319, t11321, t11322, t11326, t11328, t11329, t11331, t11332, t11334) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1086::<F>(t11315, t923, t11156, t2908, t141, t11165, t930, t2912, t698, t11151, t11160, t11132);
    (t11316, t11318, t11319, t11321, t11322, t11326, t11328, t11329, t11331, t11332, t11334)
}
