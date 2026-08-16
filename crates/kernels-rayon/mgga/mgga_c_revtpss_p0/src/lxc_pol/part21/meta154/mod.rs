//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta154 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk981;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk982;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk983;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk984;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk985;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk986;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk987;
use chunk7::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk988;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta154(t1244: f64, t3598: f64, t3594: f64, t3153: f64, t471: f64, t3602: f64, t1042: f64, t1121: f64, t414: f64, t66: f64, t3363: f64, t247: f64, t474: f64, t479: f64, t3089: f64, t1285: f64, t1264: f64, t828: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3609, t3610, t3611) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk981(t1244, t3598, t3594, t3153, t471);
        let (t3612, t3613) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk982(t3602, t3611, t1042);
        let t3617 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk983(t1121, t414);
        let (t3618, t3620) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk984(t3617, t66, t3363, t247);
        let t3623 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk985(t474, t479);
        let t3624 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk986(t3089, t3623);
        let t3625 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk987(t1285, t3624);
        let t3626 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk988(t1264, t828);
    (t3609, t3610, t3611, t3612, t3613, t3617, t3618, t3620, t3623, t3624, t3625, t3626)
}
