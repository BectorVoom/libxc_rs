//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta28 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk200;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk201;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk202;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk203;
use chunk4::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk204;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta28(t213: f64, t547: f64, t531: f64, t241: f64, t247: f64, t217: f64, t535: f64, t225: f64, t546: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t548, t549, t550) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk200(t213, t547, t531);
        let (t552, t555) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk201(t241, t550, t247, t217, t535, t548);
        let t556 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk202(t225, t555);
        let (t557, t560, t561) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk203(t546, t555, t213);
        let (t562, t565, t566) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk204(t556, t561, t213);
    (t548, t549, t550, t552, t555, t556, t557, t560, t561, t562, t565, t566)
}
