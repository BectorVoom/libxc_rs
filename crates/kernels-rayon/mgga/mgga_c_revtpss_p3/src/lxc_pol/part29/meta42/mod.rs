//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta42 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk269;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk270;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk271;
use chunk3::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk272;
use chunk4::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk273;
use chunk5::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk274;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta42(t234: f64, t243: f64, t808: f64, t807: f64, t236: f64, t786: f64, t240: f64, t27: f64, t124: f64, t800: f64, t213: f64, t225: f64, t232: f64, t235: f64, t239: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t810, t812, t813, t814) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk269(t234, t243, t808, t807, t236, t786, t240, t27);
        let (t815, t816) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk270(t243, t814, t124, t800);
        let (t817, t819, t820) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk271(t815, t816, t813, t213, t225);
        let (t821, t822) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk272(t232);
        let t823 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk273(t235, t822);
        let t825 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk274(t239, t820, t823);
    (t810, t812, t813, t814, t816, t817, t819, t820, t821, t822, t823, t825)
}
