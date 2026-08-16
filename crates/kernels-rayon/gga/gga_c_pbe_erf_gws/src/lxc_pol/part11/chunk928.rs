//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 928/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk928(t120: f64, t133: f64, t4573: f64, t1473: f64, t1497: f64, t5615: f64, t751: f64, t1332: f64, t296: f64, t6073: f64, t2059: f64, t2060: f64, t279: f64, t6045: f64) -> (f64, f64, f64, f64, f64) {
    let t19439 = 0.29801938271604938271e1_f64 * t133 * t4573 * t120;
    let t19458 = 0.31931290694012290916e0_f64 * t1473 * t1497;
    let t19466 = 0.79828226735030727292e-1_f64 * t751 * t5615;
    let t19482 = 0.47400060215270560269e1_f64 * t6073 * t1332 * t296;
    let t19517 = 0.16521134411652656606e2_f64 * t2059 * t2060 * t6045 * t279;
    (t19439, t19458, t19466, t19482, t19517)
}
