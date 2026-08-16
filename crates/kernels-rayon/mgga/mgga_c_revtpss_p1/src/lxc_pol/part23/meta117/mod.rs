//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta117 (260520-c91 hierarchical CSE).
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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk760;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk761;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk762;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk763;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk764;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk765;
use chunk6::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk766;
use chunk7::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk767;
use chunk8::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk768;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta117(t1052: f64, t369: f64, t361: f64, t351: f64, t1065: f64, t126: f64, t906: f64, t247: f64, t1063: f64, t1086: f64, t994: f64, t3090: f64, t373: f64, t66: f64, t828: f64, t1032: f64, t989: f64, t1040: f64, t1024: f64, t1062: f64, t1031: f64, t196: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t3105 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk760(t1052, t369, t361);
        let t3106 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk761(t3105, t351);
        let t3109 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk762(t1065, t126);
        let (t3111, t3112, t3114, t3115) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk763(t3109, t906, t247, t1063, t1086, t994, t3090);
        let t3116 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk764(t373, t66);
        let t3117 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk765(t3116, t828);
        let t3124 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk766(t1032, t989, t1040);
        let t3127 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk767(t1024, t1062);
        let t3140 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk768(t1031, t196);
    (t3105, t3106, t3109, t3111, t3112, t3114, t3115, t3116, t3117, t3124, t3127, t3140)
}
