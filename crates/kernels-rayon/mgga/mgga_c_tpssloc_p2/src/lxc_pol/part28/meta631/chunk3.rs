//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1980/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1980(t87291: f64, t87293: f64, t87300: f64, t87304: f64, t87308: f64, t81857: f64, t81859: f64, t81874: f64, t81877: f64, t81883: f64, t87287: f64, t87289: f64, t87296: f64, t87298: f64, t87306: f64, t87312: f64, t87316: f64, t87322: f64) -> f64 {
    let t92626 = 7.0_f64 / 36.0_f64 * t87291;
    let t92627 = 0.33913115119077928316e-1_f64 * t87293;
    let t92630 = 35.0_f64 / 144.0_f64 * t87300;
    let t92633 = 35.0_f64 / 108.0_f64 * t87304;
    let t92635 = 0.33913115119077928316e-1_f64 * t87308;
    let t92642 = -5.0_f64 / 96.0_f64 * t87287 + t87289 / 96.0_f64 + t92626 + t92627 - t87296 / 384.0_f64 - t87298 / 768.0_f64 - t92630 - 35.0_f64 / 288.0_f64 * t81857 + 0.28260929265898273597e-2_f64 * t81859 - t92633 - 0.13565246047631171326e0_f64 * t87306 - t92635 + 0.80745512188280781706e-3_f64 * t87312 + 0.48447307312968469024e-2_f64 * t87316 + 0.67287926823567318088e-4_f64 * t81874 + 0.67287926823567318088e-4_f64 * t81877 - 0.21083550404717759668e-2_f64 * t81883 - t87322 / 96.0_f64;
    t92642
}
