//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta336 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1794;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta336<F: Float>(t11044: F, t2467: F, t2828: F, t676: F, t123: F, t2465: F, t2410: F, t261: F, t2832: F, t892: F, t2408: F, t2411: F) -> (F, F, F, F, F, F, F) {
        let (t11045, t11049, t11050, t11051, t11064, t11075, t11084) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1794::<F>(t11044, t2467, t2828, t676, t123, t2465, t2410, t261, t2832, t892, t2408, t2411);
    (t11045, t11049, t11050, t11051, t11064, t11075, t11084)
}
