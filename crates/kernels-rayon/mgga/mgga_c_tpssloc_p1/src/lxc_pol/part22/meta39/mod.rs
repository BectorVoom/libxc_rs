//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta39 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk276;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk277;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk278;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk279;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk280;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk281;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk282;
use chunk7::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk283;
use chunk8::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk284;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta39(t40: f64, t607: f64, t767: f64, t76: f64, zeta_threshold: f64, t52: f64, t583: f64, t60: f64, t59: f64, t207: f64, t215: f64, t154: f64, t229: f64, t205: f64, t210: f64, t214: f64, t16: f64, t120: f64, t212: f64, t118: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t770, t771) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk276(t40, t607, t767, t76, zeta_threshold);
        let t776 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk277(t52, t607, t771, t770, zeta_threshold);
        let t781 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk278(t583, t60);
        let t782 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk279(t59, t781);
        let (t785, t786) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk280(t207, t215, t782, t154, t229);
        let t787 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk281(t205, t786);
        let (t789, t792) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk282(t210, t214, t776, t16, t59);
        let t794 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk283(t120, t212);
        let t795 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk284(t118, t794);
    (t771, t776, t781, t782, t785, t786, t787, t789, t792, t794, t795)
}
