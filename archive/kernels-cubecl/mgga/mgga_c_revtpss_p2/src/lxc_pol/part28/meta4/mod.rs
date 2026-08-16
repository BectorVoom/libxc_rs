//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta4 (260520-c91 hierarchical CSE).
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
mod chunk9;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk28;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk29;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk30;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk31;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk32;
use chunk5::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk33;
use chunk6::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk34;
use chunk7::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk35;
use chunk8::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk36;
use chunk9::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk37;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta4<F: Float>(t65: F, t16: F, t64: F, t44: F, t49: F, t56: F, t61: F, t38: F, t45: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let t66 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk28::<F>(t65);
        let t68 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk29::<F>(t16, t66);
        let t69 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk30::<F>(t64, t68);
        let t70 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk31::<F>(t44, t49, t56, t61, t69);
        let t71 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk32::<F>(t38, t70);
        let t72 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk33::<F>();
        let t73 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk34::<F>();
        let t76 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk35::<F>(t73);
        let t77 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk36::<F>(t72, t76);
        let t78 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk37::<F>(t45);
    (t66, t68, t69, t70, t71, t72, t73, t76, t77, t78)
}
