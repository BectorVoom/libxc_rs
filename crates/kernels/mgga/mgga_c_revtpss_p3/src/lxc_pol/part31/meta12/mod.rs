//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta12 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk86;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk87;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk88;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk89;
use chunk4::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk90;
use chunk5::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk91;
use chunk6::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk92;
use chunk7::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk93;
use chunk8::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk94;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta12<F: Float>(t128: F, t16: F, t65: F, t64: F, t159: F, t206: F, t122: F, t124: F, t136: F, t196: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t209 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk86::<F>(t128);
        let (t211, t212) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk87::<F>(t128);
        let t213 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk88::<F>(t209, t212);
        let t215 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk89::<F>(t16, t65);
        let t216 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk90::<F>(t215, t64);
        let (t217, t218) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk91::<F>(t159, t216, t206);
        let t220 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk92::<F>(t122);
        let t221 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk93::<F>(t124, t220);
        let (t222, t225) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk94::<F>(t136, t218, t221, t196);
    (t209, t211, t212, t213, t215, t216, t217, t218, t220, t221, t222, t225)
}
