//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta600 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1935;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta600<F: Float>(t29682: F, t689: F, t1032: F, t6041: F, t867: F, t786: F, t18451: F, t25270: F, t18462: F, t18647: F, t18527: F, t98988: F) -> (F, F, F, F, F, F, F, F) {
        let (t105936, t105944, t105945, t105946, t105985, t105987, t105989, t105991) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1935::<F>(t29682, t689, t1032, t6041, t867, t786, t18451, t25270, t18462, t18647, t18527, t98988);
    (t105936, t105944, t105945, t105946, t105985, t105987, t105989, t105991)
}
