//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta125 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk673;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk674;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk675;
use chunk3::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk676;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta125<F: Float>(t1043: F, t73: F, t357: F, t905: F, t606: F, t1052: F, t369: F, t361: F, t351: F, t1065: F, t126: F, t906: F, t247: F, t1063: F, t1086: F, t994: F, t3090: F, t373: F, t66: F, t828: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t3093, t3094, t3095, t3105, t3106, t3109) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk673::<F>(t1043, t73, t357, t905, t606, t1052, t369, t361, t351, t1065, t126);
        let (t3111, t3112, t3114, t3115) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk674::<F>(t3109, t906, t247, t1063, t1086, t994, t3090);
        let t3116 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk675::<F>(t373, t66);
        let t3117 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk676::<F>(t3116, t828);
    (t3093, t3094, t3095, t3105, t3106, t3109, t3111, t3112, t3114, t3115, t3116, t3117)
}
