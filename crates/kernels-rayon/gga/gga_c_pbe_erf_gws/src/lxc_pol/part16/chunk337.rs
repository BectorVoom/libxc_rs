//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 337/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk337(t1006: f64, t199: f64, t591: f64, t950: f64, t590: f64, t587: f64, t1000: f64, t606: f64, t1002: f64, t25: f64, t599: f64, t604: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1008 = 2.0_f64 / 15.0_f64 * t1006 * t199;
    let t1009 = t591 * t950;
    let t1010 = t590 * t1009;
    let t1012 = 4.0_f64 / 45.0_f64 * t587 * t1010;
    let t1014 = t606 * t1000;
    let t1017 = -t599 - 0.35991666666666666667e-1_f64 * t1002 - t604 - 0.66666666666666666667e-2_f64 * t25 * t1014;
    (t1008, t1009, t1010, t1012, t1014, t1017)
}
