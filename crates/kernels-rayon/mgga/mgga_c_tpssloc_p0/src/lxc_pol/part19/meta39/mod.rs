//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta39 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk272;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk273;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk274;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk275;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk276;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta39(t120: f64, t212: f64, t118: f64, t207: f64, t792: f64, t785: f64, t787: f64, t789: f64, t252: f64, t154: f64, t782: f64, t222: f64, t119: f64, t776: f64, t210: f64, t225: f64, t237: f64, t226: f64, t68: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t794 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk272(t120, t212);
        let t795 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk273(t118, t794);
        let t798 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk274(t207, t792, t795, t785, t787, t789);
        let (t799, t801, t803, t804, t805, t808) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk275(t252, t798, t154, t782, t222, t119, t776, t210, t225);
        let (t809, t812) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk276(t237, t808, t226, t68);
    (t794, t795, t798, t799, t801, t803, t804, t805, t808, t809, t812)
}
