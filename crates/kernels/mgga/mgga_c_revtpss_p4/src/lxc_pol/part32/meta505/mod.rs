//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta505 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1793;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta505<F: Float>(t33: F, t5962: F, t6079: F, t1583: F, t1711: F, t6075: F, t25826: F, t5891: F, t5915: F, t6998: F, t6846: F, t7264: F) -> (F, F, F, F, F, F, F) {
        let (t29953, t29964, t29967, t29970, t29999, t30001, t30035) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1793::<F>(t33, t5962, t6079, t1583, t1711, t6075, t25826, t5891, t5915, t6998, t6846, t7264);
    (t29953, t29964, t29967, t29970, t29999, t30001, t30035)
}
