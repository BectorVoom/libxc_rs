//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta36 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk241;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk242;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk243;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk244;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk245;
use chunk5::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk246;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta36(t128: f64, t72: f64, t686: f64, t3: f64, t66: f64, t124: f64, t138: f64, t687: f64, t689: f64, t146: f64, t682: f64, t36: f64, t37: f64, t157: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t692, t693, t696, t697) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk241(t128, t72, t686, t3, t66, t124);
        let t698 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk242(t138, t697);
        let (t700, t701) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk243(t687, t689, t693, t698, t146);
        let (t702, t704) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk244(t700, t701, t682);
        let t705 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk245(t36, t37);
        let t706 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk246(t157, t705);
    (t692, t693, t696, t697, t698, t700, t701, t702, t704, t705, t706)
}
