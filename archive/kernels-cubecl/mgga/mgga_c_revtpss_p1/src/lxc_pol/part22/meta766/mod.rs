//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta766 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2848;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta766<F: Float>(t12077: F, t989: F, t12153: F, t3057: F, t3043: F, t3316: F, t1071: F, t11200: F, t378: F, t42358: F, t11223: F, t3376: F, t3383: F) -> (F, F, F, F, F, F, F) {
        let (t43574, t43598, t43611, t43637, t43642, t43656, t43748) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2848::<F>(t12077, t989, t12153, t3057, t3043, t3316, t1071, t11200, t378, t42358, t11223, t3376, t3383);
    (t43574, t43598, t43611, t43637, t43642, t43656, t43748)
}
