//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1058/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1058(t3123: f64, t9028: f64, t3128: f64, t8869: f64, t3111: f64, t3786: f64, t850: f64, t860: f64, t2848: f64, t339: f64, t1123: f64, t11651: f64, t4386: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11874 = t3123 * t9028 / 48.0_f64;
    let t11876 = t3128 * t8869 / 8.0_f64;
    let t11878 = t850 * t3111 * t3786;
    let t11880 = t11878 * t860 / 96.0_f64;
    let t11881 = t2848 * t339;
    let t11883 = t850 * t1123 * t11881;
    let t11885 = t11883 * t860 / 96.0_f64;
    let t11886 = t4386 * t11651;
    (t11874, t11876, t11878, t11880, t11883, t11885, t11886)
}
