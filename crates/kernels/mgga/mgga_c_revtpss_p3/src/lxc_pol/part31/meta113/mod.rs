//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta113 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk662;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta113<F: Float>(t240: F, t2681: F, t243: F, t247: F, t237: F, t124: F, t212: F, t596: F, t800: F) -> (F, F, F, F) {
        let (t2682, t2684, t2686, t2689) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk662::<F>(t240, t2681, t243, t247, t237, t124, t212, t596, t800);
    (t2682, t2684, t2686, t2689)
}
