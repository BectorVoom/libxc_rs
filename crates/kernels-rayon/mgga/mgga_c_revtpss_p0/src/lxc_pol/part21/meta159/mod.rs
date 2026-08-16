//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta159 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1009;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1010;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1011;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1012;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1013;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1014;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1015;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta159(t1214: f64, t1263: f64, t1122: f64, t1042: f64, t1209: f64, t1284: f64, t3624: f64, t482: f64, t66: f64, t828: f64, t1248: f64, t1250: f64, t1222: f64, t1235: f64, t1238: f64, t1252: f64, t3663: f64, t3667: f64, t3671: f64, t3674: f64, t3679: f64, t3684: f64, t3686: f64, t3689: f64, t3694: f64, t3701: f64, t3705: f64, t3708: f64, t3711: f64, t3660: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3712, t3713, t3714) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1009(t1214, t1263, t1122, t1042);
        let (t3717, t3718) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1010(t1209, t1284, t3624);
        let t3719 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1011(t482, t66);
        let t3720 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1012(t3719, t828);
        let (t3721, t3722, t3723) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1013(t1214, t1248, t1250, t3720);
        let t3726 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1014(t1222, t1235, t1238, t1252, t3663, t3667, t3671, t3674, t3679, t3684, t3686, t3689, t3694, t3701, t3705, t3708, t3711, t3714, t3718, t3723);
        let t3727 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1015(t3660, t3726);
    (t3712, t3713, t3714, t3717, t3718, t3719, t3720, t3721, t3722, t3723, t3727)
}
