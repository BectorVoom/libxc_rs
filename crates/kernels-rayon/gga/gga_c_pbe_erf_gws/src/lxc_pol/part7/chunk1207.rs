//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1207/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1207(t20618: f64, t2157: f64, t2289: f64, t6234: f64, t2142: f64, t6621: f64, t2319: f64, t6262: f64, t2080: f64, t20807: f64, t2083: f64, t2085: f64, t860: f64) -> (f64, f64, f64, f64, f64) {
    let t21447 = t20618 * t2157;
    let t21452 = t2289 * t6234;
    let t21454 = t6621 * t2142;
    let t21455 = 7.0_f64 / 72.0_f64 * t21454;
    let t21456 = t2319 * t6262;
    let t21462 = t2080 * t20807 * t2083 * t2085 * t860 / 32.0_f64;
    (t21447, t21452, t21455, t21456, t21462)
}
