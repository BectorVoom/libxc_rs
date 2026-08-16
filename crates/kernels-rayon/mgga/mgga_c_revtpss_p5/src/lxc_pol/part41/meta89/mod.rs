//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta89 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk517;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk518;
use chunk2::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk519;
use chunk3::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk520;
use chunk4::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk521;
use chunk5::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk522;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta89(t1916: f64, t1918: f64, t572: f64, t573: f64, t76: f64, t84: f64, t198: f64, t207: f64, t159: f64, t215: f64, t655: f64, t96: f64, t114: f64, t101: f64, t69: f64, t508: f64, t569: f64, t1312: f64, t651: f64, t3: f64, param_d: f64, t117: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1921, t1927, t1940, t1941, t2174) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk517(t1916, t1918, t572, t573, t76, t84, t198, t207, t159, t215, t655, t96);
        let (t2175, t2178) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk518(t114, t101, t2174, t69);
        let t2179 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk519(t2178, t508);
        let t2181 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk520(t2178, t569);
        let (t2184, t2185, t2187) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk521(t1312, t2179, t2181, t651, t3, param_d);
        let t2189 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk522(t117, t2178);
    (t1921, t1927, t1940, t1941, t2175, t2178, t2179, t2181, t2184, t2185, t2187, t2189)
}
