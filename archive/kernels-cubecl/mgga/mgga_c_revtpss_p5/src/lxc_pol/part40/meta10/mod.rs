//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta10 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk69;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk70;
use chunk2::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk71;
use chunk3::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk72;
use chunk4::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk73;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta10<F: Float>(t128: F, t131: F, t134: F, t141: F, t149: F, t164: F, t162: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t169, t172, t173, t177) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk69::<F>(t128, t131, t134, t141);
        let (t182, t185, t186) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk70::<F>(t128, t131, t134, t141);
        let t187 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk71::<F>(t177, t186);
        let t189 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk72::<F>(t149, t164, t173, t187);
        let t190 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk73::<F>(t162, t189);
    (t169, t172, t173, t177, t182, t185, t186, t187, t189, t190)
}
