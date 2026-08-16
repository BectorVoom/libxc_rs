//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta158 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;
mod chunk9;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1048;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1049;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1050;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1051;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1052;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1053;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1054;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1055;
use chunk8::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1056;
use chunk9::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1057;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta158(t482: f64, t66: f64, t828: f64, t1214: f64, t1248: f64, t1250: f64, t1222: f64, t1235: f64, t1238: f64, t1252: f64, t3663: f64, t3667: f64, t3671: f64, t3674: f64, t3679: f64, t3684: f64, t3686: f64, t3689: f64, t3694: f64, t3701: f64, t3705: f64, t3708: f64, t3711: f64, t3714: f64, t3718: f64, t3660: f64, t225: f64, t494: f64, t1269: f64, t460: f64, t1275: f64, t493: f64, t1294: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t3719 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1048(t482, t66);
        let t3720 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1049(t3719, t828);
        let (t3721, t3722, t3723) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1050(t1214, t1248, t1250, t3720);
        let t3726 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1051(t1222, t1235, t1238, t1252, t3663, t3667, t3671, t3674, t3679, t3684, t3686, t3689, t3694, t3701, t3705, t3708, t3711, t3714, t3718, t3723);
        let t3727 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1052(t3660, t3726);
        let (t3729, t3732) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1053(t225, t3727, t494, t1269, t460);
        let t3736 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1054(t1275, t493);
        let t3737 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1055(t225, t3736);
        let t3738 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1056(t1294);
        let t3739 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1057(t3737, t3738);
    (t3719, t3720, t3721, t3722, t3723, t3727, t3729, t3732, t3736, t3737, t3738, t3739)
}
