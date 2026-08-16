//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta112 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk756;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk757;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk758;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk759;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk760;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk761;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk762;
use chunk7::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk763;
use chunk8::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk764;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta112(t1057: f64, t3112: f64, t3032: f64, t3127: f64, t3031: f64, t1932: f64, t3131: f64, t1014: f64, t360: f64, t390: f64, t1878: f64, t268: f64, t405: f64, t1091: f64, t690: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t3180 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk756(t1057, t3112);
        let t3185 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk757(t3032, t3127);
        let t3186 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk758(t3031, t3185);
        let t3188 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk759(t1932, t3131);
        let t3199 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk760(t1014, t3032);
        let t3200 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk761(t3031, t3199);
        let t3201 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk762(t1932, t360);
        let (t3215, t3216) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk763(t390);
        let (t3236, t3237, t3238) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk764(t1878, t268, t405, t1091, t690);
    (t3180, t3185, t3186, t3188, t3199, t3200, t3201, t3215, t3216, t3236, t3237, t3238)
}
