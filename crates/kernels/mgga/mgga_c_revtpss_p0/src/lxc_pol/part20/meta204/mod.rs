//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta204 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk976;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta204<F: Float>(t10190: F, t9397: F, t9557: F, t9589: F, t2327: F, t648: F, t64: F, t843: F, t112: F, t2289: F, t666: F, t2341: F, t625: F) -> (F, F, F, F, F, F) {
        let (t10192, t10194, t10199, t10201, t10202, t10204) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk976::<F>(t10190, t9397, t9557, t9589, t2327, t648, t64, t843, t112, t2289, t666, t2341, t625);
    (t10192, t10194, t10199, t10201, t10202, t10204)
}
