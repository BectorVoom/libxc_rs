//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 840/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk840(t418: f64, t7056: f64, t7063: f64, t7062: f64, t1660: f64, t597: f64, t1663: f64, t2647: f64, t723: f64, t2650: f64, t4985: f64, t4993: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7064 = t7056 * t418;
    let t7065 = t7063 * t7064;
    let t7067 = 16.0_f64 / 45.0_f64 * t7062 * t7065;
    let t7068 = t1660 * t597;
    let t7069 = t7068 * t1663;
    let t7070 = t7069 * t7064;
    let t7072 = 8.0_f64 / 27.0_f64 * t7062 * t7070;
    let t7074 = 4.0_f64 / 9.0_f64 * t2647 * t723;
    let t7075 = t2650 * t723;
    let t7077 = 8.0_f64 / 45.0_f64 * t4985;
    let t7079 = 16.0_f64 / 405.0_f64 * t4993;
    (t7067, t7072, t7074, t7075, t7077, t7079)
}
