//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta98 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk623;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk624;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk625;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk626;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta98(t2251: f64, t70: f64, t2: f64, t580: f64, t17: f64, t30: f64, t33: f64, zeta_threshold: f64, t36: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t2252, t2255) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk623(t2251, t70, t2, t580);
        let (t2256, t2257) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk624(t17, t2255);
        let t2258 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk625(t30, t33, t2257, zeta_threshold);
        let t2259 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk626(t2258, t36);
    (t2252, t2255, t2256, t2257, t2258, t2259)
}
