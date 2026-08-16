//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 813/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk813(t2096: f64, t2454: f64, t4498: f64, t19: f64, t3025: f64, t796: f64, t801: f64, t1105: f64, t945: f64, t810: f64, t2474: f64, t460: f64) -> (f64, f64, f64, f64, f64) {
    let t6906 = t2454 * t2096;
    let t6918 = 4.0_f64 * t4498;
    let t6921 = t3025 * t796 * t19;
    let t6922 = t6921 * t801;
    let t6923 = 0.82152657680133333336e0_f64 * t6922;
    let t6925 = t945 * t1105;
    let t6926 = t6925 * t810;
    let t6930 = t2474 * t460;
    (t6906, t6918, t6923, t6926, t6930)
}
