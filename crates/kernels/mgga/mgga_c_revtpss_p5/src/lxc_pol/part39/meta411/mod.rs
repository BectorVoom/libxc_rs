//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta411 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1488;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta411<F: Float>(t1464: F, t8283: F, t10208: F, t625: F, t31036: F, t31027: F, t31040: F, t31032: F, t31059: F, t46157: F, t69: F, t2289: F, t2339: F) -> (F, F, F, F, F, F, F) {
        let (t116899, t116912, t116913, t116915, t116917, t116919, t116926) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1488::<F>(t1464, t8283, t10208, t625, t31036, t31027, t31040, t31032, t31059, t46157, t69, t2289, t2339);
    (t116899, t116912, t116913, t116915, t116917, t116919, t116926)
}
