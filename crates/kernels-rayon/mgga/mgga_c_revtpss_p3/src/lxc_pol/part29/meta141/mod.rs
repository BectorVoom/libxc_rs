//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta141 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk734;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk735;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk736;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta141(t1043: f64, t73: f64, t357: f64, t905: f64, t606: f64, t3092: f64, t1066: f64, t2858: f64, t247: f64, t1052: f64, t369: f64, t361: f64, t351: f64, t1065: f64, t126: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3093, t3094, t3095, t3096, t3097, t3101, t3105) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk734(t1043, t73, t357, t905, t606, t3092, t1066, t2858, t247, t1052, t369, t361);
        let t3106 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk735(t3105, t351);
        let t3109 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk736(t1065, t126);
    (t3093, t3094, t3095, t3096, t3097, t3101, t3105, t3106, t3109)
}
