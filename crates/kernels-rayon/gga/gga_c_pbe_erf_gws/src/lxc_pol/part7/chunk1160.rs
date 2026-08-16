//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1160/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1160(t2289: f64, t6497: f64, t2168: f64, t2195: f64, t3139: f64, t6269: f64, t2156: f64, t2157: f64, t2155: f64, t858: f64, t867: f64, t2251: f64, t2300: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t20720 = t2289 * t6497;
    let t20725 = t2168 * t3139 * t6269 * t2195 / 16.0_f64;
    let t20726 = t2156 * t2156;
    let t20727 = t20726 * t2157;
    let t20731 = 7.0_f64 / 48.0_f64 * t2155 * t867 * t858 * t20727;
    let t20732 = t2251 * t2300;
    (t20720, t20725, t20726, t20727, t20731, t20732)
}
