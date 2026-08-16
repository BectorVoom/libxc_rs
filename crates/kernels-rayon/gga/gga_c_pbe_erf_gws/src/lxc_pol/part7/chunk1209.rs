//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1209/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1209(t1452: f64, t810: f64, t4422: f64, t885: f64, t2149: f64, t2146: f64, t6406: f64, t6416: f64, t6158: f64, t6670: f64, t822: f64, t20480: f64, t3065: f64, t858: f64) -> (f64, f64, f64, f64, f64) {
    let t21482 = t1452 * t810;
    let t21491 = t4422 * t885;
    let t21492 = t21491 * t2149;
    let t21493 = t2146 * t21492;
    let t21494 = 35.0_f64 / 18.0_f64 * t21493;
    let t21495 = t6416 * t6406;
    let t21497 = t6158 * t6670;
    let t21498 = t822 * t21497;
    let t21500 = t3065 * t858 * t20480;
    (t21482, t21494, t21495, t21498, t21500)
}
