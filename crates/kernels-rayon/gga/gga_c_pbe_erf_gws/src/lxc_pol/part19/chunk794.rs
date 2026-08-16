//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 794/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk794(t6055: f64, t6056: f64, t1492: f64, t751: f64, t1497: f64, t309: f64, t310: f64, t311: f64, t305: f64, t296: f64, t413: f64, t2092: f64, t2096: f64) -> (f64, f64, f64, f64, f64) {
    let t6058 = 0.45692190944741466895e-5_f64 * t6055 * t6056;
    let t6061 = t751 * t1492;
    let t6064 = 0.59871170051273045469e-1_f64 * t751 * t1497;
    let t6072 = 1.0_f64 / t311 / t310 / t309;
    let t6073 = t305 * t6072;
    let t6074 = t413 * t296;
    let t6075 = t6073 * t6074;
    let t6076 = 0.47400060215270560269e0_f64 * t6075;
    let t6080 = t2092 * t2096;
    (t6058, t6061, t6064, t6076, t6080)
}
