//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1075 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3855;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1075<F: Float>(t48287: F, t48290: F, t48292: F, t48294: F, t187: F, t73472: F, t48297: F, t48299: F, t48302: F, t48304: F, t48306: F, t47089: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t74114, t74115, t74116, t74117, t74119, t74120, t74121, t74122, t74123, t74124, t74125) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3855::<F>(t48287, t48290, t48292, t48294, t187, t73472, t48297, t48299, t48302, t48304, t48306, t47089);
    (t74114, t74115, t74116, t74117, t74119, t74120, t74121, t74122, t74123, t74124, t74125)
}
