//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta257 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk986;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk987;
use chunk2::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk988;
use chunk3::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk989;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta257<F: Float>(t2580: F, t680: F, t130: F, t146: F, t2583: F, t9275: F, t2514: F, t2596: F, t746: F, t1340: F, t2491: F, t2495: F, t744: F, t215: F, t681: F, t268: F, t702: F, t2564: F, t2567: F, t675: F, t30: F, t525: F, t2: F, t22: F, t33: F, t527: F, t2490: F, t737: F, t2492: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t9316 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk986::<F>(t2580, t680, t130, t146, t2583, t9275);
        let (t9318, t9320, t9323, t9325, t9329) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk987::<F>(t2514, t2596, t746, t1340, t2491, t2495, t744, t215, t681, t268, t702);
        let t9333 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk988::<F>(t2564, t2567, t268, t675);
        let (t9335, t9342, t9350, t9367, t9368) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk989::<F>(t30, t525, t2, t22, t33, t527, t2490, t737, t2492, t744);
    (t9316, t9318, t9320, t9323, t9325, t9329, t9333, t9335, t9342, t9350, t9367, t9368)
}
