//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta96 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk618;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk619;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk620;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk621;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta96(t20: f64, t27: f64, t12: f64, t19: f64, t592: f64, t596: f64, t21: f64, t25: f64, t599: f64, t602: f64, t89: f64, t90: f64, t29: f64, t2: f64, t580: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2230, t2231, t2233, t2235, t2236, t2237, t2239, t2242) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk618(t20, t27, t12, t19, t592, t596, t21, t25, t599, t602);
        let t2246 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk619(t89, t90);
        let t2247 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk620(t2246, t29);
        let t2255 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk621(t2, t580);
    (t2230, t2231, t2233, t2235, t2236, t2237, t2239, t2242, t2246, t2247, t2255)
}
