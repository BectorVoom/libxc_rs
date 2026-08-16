//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta12 (260520-c91 hierarchical CSE).
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
mod chunk10;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk81;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk82;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk83;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk84;
use chunk4::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk85;
use chunk5::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk86;
use chunk6::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk87;
use chunk7::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk88;
use chunk8::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk89;
use chunk9::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk90;
use chunk10::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk91;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta12(t10: f64, t60: f64, t59: f64, t201: f64, t154: f64, t117: f64, t131: f64, t136: f64, t119: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t204 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk81(t10, t60);
        let t205 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk82(t204, t59);
        let t206 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk83(t201);
        let t207 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk84(t154, t206);
        let t209 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk85(t117);
        let t210 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk86(t131, t209);
        let t212 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk87(t136);
        let t213 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk88(t212);
        let t214 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk89(t119, t213);
        let t215 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk90(t210, t214);
        let t218 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk91(t205, t207, t215);
    (t204, t205, t206, t207, t209, t210, t212, t213, t214, t215, t218)
}
