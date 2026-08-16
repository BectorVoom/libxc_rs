//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1255/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1255(t27805: f64, t45306: f64, t12065: f64, t46327: f64, t45753: f64, t45767: f64, t2157: f64, t49847: f64, t2155: f64, t858: f64, t867: f64, t3180: f64, t46199: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t49921 = t27805 * t45306 / 4.0_f64;
    let t49928 = 7.0_f64 / 48.0_f64 * t46327 * t12065;
    let t49929 = 7.0_f64 / 24.0_f64 * t45753;
    let t49931 = 7.0_f64 / 24.0_f64 * t45767;
    let t49932 = t49847 * t2157;
    let t49936 = 7.0_f64 / 48.0_f64 * t2155 * t867 * t858 * t49932;
    let t49943 = t46199 * t3180 / 6.0_f64;
    (t49921, t49928, t49929, t49931, t49932, t49936, t49943)
}
