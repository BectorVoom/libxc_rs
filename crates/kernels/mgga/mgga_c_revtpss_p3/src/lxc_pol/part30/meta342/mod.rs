//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta342 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1356;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta342<F: Float>(t2966: F, t307: F, t302: F, t11132: F, t11337: F, t944: F, t2969: F, t310: F, t2979: F, t964: F, t3011: F, t960: F) -> (F, F, F, F, F, F, F) {
        let (t11409, t11422, t11423, t11450, t11452, t11456, t11461) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1356::<F>(t2966, t307, t302, t11132, t11337, t944, t2969, t310, t2979, t964, t3011, t960);
    (t11409, t11422, t11423, t11450, t11452, t11456, t11461)
}
