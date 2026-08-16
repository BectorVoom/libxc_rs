//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta511 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2262;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2263;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2264;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2265;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2266;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2267;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2268;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2269;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta511(t16725: f64, t3360: f64, t128: f64, t2258: f64, t5046: f64, t2251: f64, t1120: f64, t3367: f64, t4186: f64, t606: f64, t5051: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16726, t16727) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2262(t16725, t3360, t128);
        let t16729 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2263(t2258, t5046);
        let (t16730, t16731) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2264(t16729, t3360, t128);
        let t16733 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2265(t2251, t5046);
        let (t16734, t16735) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2266(t1120, t16733, t128);
        let t16738 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2267(t3367, t4186, t606);
        let (t16739, t16740) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2268(t1120, t16738, t128);
        let t16742 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2269(t2258, t5051);
    (t16726, t16727, t16729, t16730, t16731, t16733, t16734, t16735, t16738, t16739, t16740, t16742)
}
