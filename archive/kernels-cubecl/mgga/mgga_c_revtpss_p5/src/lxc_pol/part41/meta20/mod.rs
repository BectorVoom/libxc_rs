//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta20 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk131;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk132;
use chunk2::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk133;
use chunk3::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk134;
use chunk4::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk135;
use chunk5::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk136;
use chunk6::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk137;
use chunk7::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk138;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta20<F: Float>(t125: F, t66: F, t283: F, t371: F, t345: F, t348: F, t367: F, t225: F, t359: F, t342: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let t372 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk131::<F>(t125, t66);
        let t373 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk132::<F>(t283);
        let t375 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk133::<F>(t371, t372, t373);
        let t378 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk134::<F>(t345, t348, t367, t375);
        let (t379, t380) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk135::<F>(t225, t378, t359);
        let t381 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk136::<F>(t378, t380);
        let (t384, t385) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk137::<F>(t342, t381);
        let t386 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk138::<F>(t379, t385);
    (t372, t373, t375, t378, t379, t380, t381, t384, t385, t386)
}
