//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta99 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk629;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk630;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk631;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta99(t2435: f64, t781: f64, t124: f64, t68: f64, t138: f64, t251: f64, t785: f64, t780: f64, t212: f64, t860: f64, t689: f64, t779: f64, t887: f64, t211: f64, t784: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2437, t2438) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk629(t2435, t781, t124, t68);
        let t2439 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk630(t138, t2438);
        let (t2440, t2441, t2443, t2444, t2445, t2446, t2448, t2449, t2452) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk631(t251, t785, t780, t2439, t212, t860, t689, t779, t887, t211, t784);
    (t2437, t2438, t2439, t2440, t2441, t2443, t2444, t2445, t2446, t2448, t2449, t2452)
}
