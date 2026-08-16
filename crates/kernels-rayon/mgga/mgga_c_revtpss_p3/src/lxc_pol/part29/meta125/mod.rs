//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta125 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk680;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk681;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk682;
use chunk3::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk683;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta125(t2770: f64, t2771: f64, t2435: f64, t871: f64, t225: f64, t785: f64, t870: f64, t2439: f64, t123: f64, t212: f64, t676: f64, t822: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let t2772 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk680(t2770, t2771);
        let (t2776, t2777) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk681(t2435, t871, t225, t785);
        let (t2778, t2780, t2782) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk682(t2777, t870, t2439, t123, t212, t676);
        let t2783 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk683(t225, t822);
    (t2772, t2776, t2777, t2778, t2780, t2782, t2783)
}
