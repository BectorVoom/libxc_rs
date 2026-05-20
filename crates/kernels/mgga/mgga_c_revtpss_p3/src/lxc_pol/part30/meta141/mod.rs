//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta141 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk761;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk762;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk763;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta141<F: Float>(t1043: F, t73: F, t357: F, t905: F, t606: F, t3092: F, t1066: F, t2858: F, t247: F, t1052: F, t369: F, t361: F, t351: F, t1065: F, t126: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t3093, t3094, t3095, t3096, t3097, t3101, t3105) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk761::<F>(t1043, t73, t357, t905, t606, t3092, t1066, t2858, t247, t1052, t369, t361);
        let t3106 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk762::<F>(t3105, t351);
        let t3109 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk763::<F>(t1065, t126);
    (t3093, t3094, t3095, t3096, t3097, t3101, t3105, t3106, t3109)
}
