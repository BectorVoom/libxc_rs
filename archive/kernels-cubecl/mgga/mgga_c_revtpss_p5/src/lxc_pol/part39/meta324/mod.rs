//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta324 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1103;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1104;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta324<F: Float>(t3369: F, t689: F, t3373: F) -> (F, F) {
        let t12301 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1103::<F>(t3369, t689);
        let t12303 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1104::<F>(t3373, t689);
    (t12301, t12303)
}
