//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta363 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1237;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1238;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1239;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1240;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1241;
use chunk5::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1242;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta363(t24232: f64, t3360: f64, t128: f64, t5046: f64, t5825: f64, t22688: f64, t3362: f64, t1120: f64, t5051: f64, t1121: f64, t22671: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t24233, t24234) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1237(t24232, t3360, t128);
        let (t24236, t24237, t24238) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1238(t5046, t5825, t3360, t128);
        let t24240 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1239(t22688, t3362);
        let (t24241, t24242) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1240(t1120, t24240, t128);
        let (t24244, t24245, t24246) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1241(t5051, t5825, t1120, t128);
        let t24248 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1242(t1121, t22671);
    (t24233, t24234, t24236, t24237, t24238, t24240, t24241, t24242, t24244, t24245, t24246, t24248)
}
