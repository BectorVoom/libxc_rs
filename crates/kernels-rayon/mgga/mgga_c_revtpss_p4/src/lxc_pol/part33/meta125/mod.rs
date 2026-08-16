//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta125 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk703;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk704;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk705;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk706;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta125(t1043: f64, t73: f64, t357: f64, t905: f64, t606: f64, t1052: f64, t369: f64, t361: f64, t351: f64, t1065: f64, t126: f64, t906: f64, t247: f64, t1063: f64, t1086: f64, t994: f64, t3090: f64, t373: f64, t66: f64, t828: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3093, t3094, t3095, t3105, t3106, t3109) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk703(t1043, t73, t357, t905, t606, t1052, t369, t361, t351, t1065, t126);
        let (t3111, t3112, t3114, t3115) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk704(t3109, t906, t247, t1063, t1086, t994, t3090);
        let t3116 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk705(t373, t66);
        let t3117 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk706(t3116, t828);
    (t3093, t3094, t3095, t3105, t3106, t3109, t3111, t3112, t3114, t3115, t3116, t3117)
}
