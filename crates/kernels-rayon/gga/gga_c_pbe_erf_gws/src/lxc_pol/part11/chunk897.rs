//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 897/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk897(t528: f64, t5420: f64, t1917: f64, t762: f64, t1472: f64, t712: f64, t713: f64, t1464: f64, t119: f64, t5559: f64, t19: f64, t5697: f64, t799: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18149 = 0.19947266666666666666e0_f64 * t528 * t5420;
    let t18155 = 0.26596355555555555555e0_f64 * t762 * t1917;
    let t18196 = 0.54024691358024691356e-1_f64 * t712 * t1472 * t713;
    let t18215 = 0.19208479012345679012e0_f64 * t1464 * t713;
    let t18224 = 0.60617527037037037035e-2_f64 * t5559 * t119 * t1917;
    let t18237 = 0.27631489407716049382e-3_f64 * t5697 * t19 * t799 * t713;
    (t18149, t18155, t18196, t18215, t18224, t18237)
}
