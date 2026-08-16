//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1267/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1267(t46251: f64, t28074: f64, t21328: f64, t50002: f64, t858: f64, t884: f64, t11600: f64, t11808: f64, t50019: f64, t866: f64, t867: f64, t46324: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t50187 = 7.0_f64 / 12.0_f64 * t46251;
    let t50189 = 455.0_f64 / 162.0_f64 * t28074;
    let t50193 = 5.0_f64 / 4.0_f64 * t884 * t21328 * t858 * t50002;
    let t50201 = t11600 * t11808 / 8.0_f64;
    let t50206 = t866 * t867 * t858 * t50019 / 96.0_f64;
    let t50207 = 7.0_f64 / 12.0_f64 * t46324;
    (t50187, t50189, t50193, t50201, t50206, t50207)
}
