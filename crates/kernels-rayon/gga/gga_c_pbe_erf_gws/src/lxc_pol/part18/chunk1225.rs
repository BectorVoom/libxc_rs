//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1225/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1225(t1176: f64, t2298: f64, t923: f64, t51649: f64, t867: f64, t3966: f64, t326: f64, t378: f64, t6594: f64, t745: f64, t837: f64, t2306: f64, t938: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t51963 = t1176 * t923 * t2298;
    let t51966 = t51649 * t867;
    let t51967 = t51966 * t3966;
    let t51977 = t326 * t6594 * t378;
    let t51978 = 455.0_f64 / 1296.0_f64 * t51977;
    let t51989 = t745 * t837;
    let t52000 = t2306 * t938;
    (t51963, t51966, t51967, t51978, t51989, t52000)
}
