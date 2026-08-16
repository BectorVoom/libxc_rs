//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta202 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1215;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1216;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1217;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1218;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1219;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1220;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1221;
use chunk7::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1222;
use chunk8::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1223;
use chunk9::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1224;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta202(t4823: f64, t906: f64, t1042: f64, t1066: f64, t4583: f64, t247: f64, t1062: f64, t1659: f64, t3204: f64, t3116: f64, t4757: f64, t127: f64, t1663: f64, t371: f64, t1025: f64, t1063: f64, t1068: f64, t1675: f64, t3106: f64, t3112: f64, t3127: f64, t3174: f64, t3188: f64, t4818: f64, t4821: f64, t373: f64, t4772: f64, t372: f64, t225: f64, t4746: f64, t366: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4824, t4825) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1215(t4823, t906, t1042);
        let t4831 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1216(t1066, t4583, t247);
        let t4834 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1217(t1062, t1659);
        let t4837 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1218(t1062, t3204);
        let t4839 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1219(t3116, t4757, t247);
        let t4845 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1220(t127, t1663, t371);
        let t4848 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1221(t1025, t4845, t1063, t1068, t1675, t3106, t3112, t3127, t3174, t3188, t4818, t4821, t4825, t4831, t4834, t4837, t4839);
        let (t4852, t4854) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1222(t373, t4772, t371, t372);
        let t4857 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1223(t225, t4746);
        let t4858 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1224(t366, t4857);
    (t4824, t4825, t4831, t4834, t4837, t4839, t4845, t4848, t4852, t4854, t4857, t4858)
}
