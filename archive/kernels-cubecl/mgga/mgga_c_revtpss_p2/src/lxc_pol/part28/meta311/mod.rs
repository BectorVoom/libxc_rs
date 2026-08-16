//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta311 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1313;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1314;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta311<F: Float>(t88: F, t89: F, t90: F, t29: F, t46: F, t47: F, t58: F, t59: F, t10199: F, t2851: F, t78: F, t3361: F, t81: F, t116: F, t2319: F) -> (F, F, F, F, F, F, F, F) {
        let (t10308, t10309) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1313::<F>(t88, t89, t90, t29);
        let (t10355, t10368, t10379, t10389, t10398, t10416) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1314::<F>(t46, t47, t58, t59, t10199, t2851, t78, t3361, t81, t116, t2319);
    (t10308, t10309, t10355, t10368, t10379, t10389, t10398, t10416)
}
