//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 643/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk643(t309: f64, t310: f64, t311: f64, t305: f64, t296: f64, t413: f64, t816: f64, t322: f64, t897: f64, t2209: f64, t337: f64, t2118: f64, t2365: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6072 = 1.0_f64 / t311 / t310 / t309;
    let t6073 = t305 * t6072;
    let t6074 = t413 * t296;
    let t6075 = t6073 * t6074;
    let t6076 = 0.47400060215270560269e0_f64 * t6075;
    let t6094 = t816 * t816;
    let t6095 = 1.0_f64 / t6094;
    let t6096 = t322 * t6095;
    let t6125 = t897 * t897;
    let t6126 = 1.0_f64 / t6125;
    let t6148 = t2209 * t337;
    let t6154 = t2118 * t2365;
    (t6072, t6073, t6074, t6075, t6076, t6094, t6095, t6096, t6125, t6126, t6148, t6154)
}
