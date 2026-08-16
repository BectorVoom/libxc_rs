//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta92 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk524;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk525;
use chunk2::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk526;
use chunk3::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk527;
use chunk4::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk528;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta92(t114: f64, t1916: f64, t1918: f64, t572: f64, t573: f64, t198: f64, t207: f64, t159: f64, t215: f64, t655: f64, t96: f64, t101: f64, t69: f64, t508: f64, t569: f64, t1312: f64, t651: f64, t3: f64, param_d: f64, t117: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1921, t1940, t1941, t2174, t2175, t2178) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk524(t114, t1916, t1918, t572, t573, t198, t207, t159, t215, t655, t96, t101, t69);
        let t2179 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk525(t2178, t508);
        let t2181 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk526(t2178, t569);
        let (t2184, t2185, t2187) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk527(t1312, t2179, t2181, t651, t3, param_d);
        let t2189 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk528(t117, t2178);
    (t1921, t1940, t1941, t2174, t2175, t2178, t2179, t2181, t2184, t2185, t2187, t2189)
}
