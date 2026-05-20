//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta350 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1159;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1160;
use chunk2::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1161;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta350<F: Float>(t5048: F, t689: F, t5053: F, t5057: F) -> (F, F, F, F) {
        let t16708 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1159::<F>(t5048, t689);
        let t16710 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1160::<F>(t5053, t689);
        let (t16711, t16712) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1161::<F>(t16710, t5057, t689);
    (t16708, t16710, t16711, t16712)
}
