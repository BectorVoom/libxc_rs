//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1220/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1220(t13917: f64, t4149: f64, t9521: f64, t14765: f64, t2118: f64, t3074: f64, t6778: f64, t13808: f64, t14754: f64, t3972: f64, t3975: f64, t9416: f64) -> (f64, f64, f64, f64) {
    let t52889 = t13917 * t4149 * t9521;
    let t52893 = t3074 * t2118 * t14765 * t6778;
    let t52901 = t13808 * t14754;
    let t52904 = t3972 * t3975 * t9416;
    (t52889, t52893, t52901, t52904)
}
