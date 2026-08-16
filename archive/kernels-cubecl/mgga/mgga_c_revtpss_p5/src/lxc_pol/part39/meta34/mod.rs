//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta34 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk215;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk216;
use chunk2::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk217;
use chunk3::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk218;
use chunk4::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk219;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta34<F: Float>(t100: F, t658: F, t108: F, t101: F, t105: F, t656: F, t97: F, t114: F, t655: F, t653: F, t69: F, t508: F, t3: F, t65: F, t125: F, t123: F, t147: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t659, t661, t662, t665) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk215::<F>(t100, t658, t108, t101, t105, t656, t97);
        let (t666, t670) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk216::<F>(t114, t655, t665, t653, t69);
        let (t671, t675) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk217::<F>(t508, t670, t3, t65);
        let t676 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk218::<F>(t125, t675);
        let t679 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk219::<F>(t123, t147, t676);
    (t659, t661, t662, t665, t666, t670, t671, t675, t676, t679)
}
