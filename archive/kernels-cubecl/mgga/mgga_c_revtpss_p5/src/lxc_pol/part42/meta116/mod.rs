//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta116 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk591;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta116<F: Float>(t913: F, t275: F) -> (F, F, F) {
        let (t2922, t2923, t2924) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk591::<F>(t913, t275);
    (t2922, t2923, t2924)
}
