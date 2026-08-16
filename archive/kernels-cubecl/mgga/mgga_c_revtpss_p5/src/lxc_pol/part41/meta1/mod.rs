//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta1 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk8;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk9;
use chunk2::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk10;
use chunk3::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk11;
use chunk4::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk12;
use chunk5::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk13;
use chunk6::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk14;
use chunk7::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk15;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta1<F: Float>(t14: F, t11: F, t16: F, t12: F, t15: F, t17: F, t9: F, t5: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t19 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk8::<F>(t14);
        let (t20, t21) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk9::<F>(t11, t19, t16);
        let t22 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk10::<F>(t21);
        let (t25, t26, t27) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk11::<F>(t12, t14, t19, t16, t21);
        let (t29, t30) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk12::<F>(t15, t17, t20, t22, t25, t27, t9, t5);
        let (t32, t33) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk13::<F>(t30, t5, zeta_threshold);
        let t36 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk14::<F>(t30, t33, t32, t5, zeta_threshold);
        let t37 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk15::<F>(t36);
    (t19, t20, t21, t22, t25, t26, t27, t29, t30, t33, t36, t37)
}
