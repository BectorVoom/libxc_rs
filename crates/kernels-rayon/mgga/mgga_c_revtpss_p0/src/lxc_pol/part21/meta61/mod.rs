//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta61 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk448;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk449;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk450;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk451;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk452;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk453;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta61(t225: f64, t494: f64, t1118: f64, t1124: f64, t139: f64, t221: f64, t462: f64, t461: f64, t1010: f64, t56: f64, t403: f64, t404: f64, t1121: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t1211 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk448(t225, t494);
        let (t1212, t1214) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk449(t1118, t1124);
        let t1215 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk450(t1211, t1214);
        let (t1219, t1221, t1222) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk451(t139, t221, t462, t461, t1010, t56);
        let t1224 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk452(t403, t404);
        let t1225 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk453(t1121, t1224);
    (t1211, t1212, t1214, t1215, t1219, t1221, t1222, t1224, t1225)
}
