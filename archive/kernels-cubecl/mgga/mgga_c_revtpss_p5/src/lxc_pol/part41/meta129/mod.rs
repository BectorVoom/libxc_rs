//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta129 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk624;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta129<F: Float>(t1129: F, t408: F) -> (F, F, F) {
        let (t3431, t3432, t3433) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk624::<F>(t1129, t408);
    (t3431, t3432, t3433)
}
