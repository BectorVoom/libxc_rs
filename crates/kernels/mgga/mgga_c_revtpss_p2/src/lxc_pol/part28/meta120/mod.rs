//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta120 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk691;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk692;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta120<F: Float>(t2689: F, t810: F, t775: F, t854: F, t236: F, t807: F, t21: F, t65: F, t64: F, t159: F, t222: F, t794: F, t798: F, t802: F, t124: F, t2430: F, t800: F, t234: F, t2453: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t2691, t2693, t2694, t2695, t2698, t2699, t2700, t2702, t2703) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk691::<F>(t2689, t810, t775, t854, t236, t807, t21, t65, t64, t159, t222, t794, t798);
        let (t2704, t2707, t2710) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk692::<F>(t2703, t802, t124, t2430, t800, t234, t2453);
    (t2691, t2693, t2694, t2695, t2698, t2699, t2700, t2702, t2703, t2704, t2707, t2710)
}
