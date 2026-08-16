//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta92 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk528;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk529;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta92<F: Float>(t599: F, t602: F, t89: F, t90: F, t29: F, t2: F, t580: F, t47: F, t59: F, t239: F, t64: F) -> (F, F, F, F, F, F, F) {
        let (t2242, t2246, t2247, t2255) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk528::<F>(t599, t602, t89, t90, t29, t2, t580);
        let (t2275, t2282, t2289) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk529::<F>(t47, t59, t239, t64);
    (t2242, t2246, t2247, t2255, t2275, t2282, t2289)
}
