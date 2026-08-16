//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta123 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk610;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk611;
use chunk2::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk612;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta123<F: Float>(t357: F, t3153: F, t1036: F, t3148: F, t3141: F, t1038: F, t1052: F, t1033: F, t127: F, t246: F, t1046: F, t1041: F, t283: F, t905: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t3154 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk610::<F>(t357);
        let (t3155, t3160, t3161, t3162, t3168, t3169, t3172) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk611::<F>(t3153, t3154, t1036, t3148, t3141, t357, t1038, t1052, t1033, t127, t246);
        let (t3173, t3174, t3181) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk612::<F>(t1046, t3172, t1041, t283, t905);
    (t3154, t3155, t3160, t3161, t3162, t3168, t3169, t3172, t3173, t3174, t3181)
}
