//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta279 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1032;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta279<F: Float>(t10308: F, t29: F, t46: F, t47: F, t58: F, t59: F, t10199: F, t2851: F, t78: F, t3361: F, t81: F, t157: F, t36: F) -> (F, F, F, F, F, F, F) {
        let (t10309, t10355, t10368, t10379, t10389, t10398, t10439) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1032::<F>(t10308, t29, t46, t47, t58, t59, t10199, t2851, t78, t3361, t81, t157, t36);
    (t10309, t10355, t10368, t10379, t10389, t10398, t10439)
}
