//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 883/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk883(t1644: f64, t1771: f64, t458: f64, t7956: f64, t7974: f64, t37315: f64, t378: f64, t92: f64, t1570: f64, t37362: f64, t37264: f64, t38077: f64, t38081: f64, t38084: f64, t38088: f64, t38090: f64, t38092: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t38094 = t1771 * t1644;
    let t38096 = t458 * t7956;
    let t38098 = t458 * t7974;
    let t38101 = t92 * t378 * t37315;
    let t38103 = t1570 * t37362;
    let t38105 = t92 * t378 * t38103;
    let t38108 = t92 * t378 * t37264;
    let t38110 = 8.0_f64 * t38077 - 2.0_f64 / 3.0_f64 * t38081 - 8.0_f64 / 9.0_f64 * t38084 + 8.0_f64 * t38088 + 112.0_f64 / 81.0_f64 * t38090 - 8.0_f64 / 9.0_f64 * t38092 - 16.0_f64 / 27.0_f64 * t38094 + 40.0_f64 / 81.0_f64 * t38096 + 4.0_f64 / 9.0_f64 * t38098 - 12.0_f64 * t38101 + 2.0_f64 * t38105 + 8.0_f64 / 3.0_f64 * t38108;
    (t38094, t38096, t38098, t38101, t38103, t38105, t38108, t38110)
}
