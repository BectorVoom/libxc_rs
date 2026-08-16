//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta98 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk565;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk566;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk567;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk568;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk569;
use chunk5::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk570;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta98(t127: f64, t246: f64, t283: f64, t905: f64, t66: f64, t371: f64, t373: f64, t676: f64, t367: f64, t225: f64, t3057: f64, t366: f64, t1014: f64, t2857: f64, t271: f64, t2852: f64, t1077: f64, t384: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t3172 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk565(t127, t246);
        let t3181 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk566(t283, t905);
        let t3182 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk567(t3181, t66);
        let (t3201, t3203, t3204) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk568(t371, t373, t676, t367, t225, t3057);
        let (t3205, t3236, t3252) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk569(t3204, t366, t1014, t2857, t271, t905);
        let (t3253, t3268, t3269) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk570(t2852, t3252, t1077, t384, t225);
    (t3172, t3181, t3182, t3201, t3203, t3204, t3205, t3236, t3252, t3253, t3268, t3269)
}
