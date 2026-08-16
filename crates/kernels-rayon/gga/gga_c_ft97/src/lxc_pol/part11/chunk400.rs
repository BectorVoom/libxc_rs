//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 400/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk400(t2157: f64, t605: f64, t144: f64, t161: f64, t1637: f64, t89: f64, t1882: f64, t576: f64, t611: f64, t558: f64, t574: f64, t616: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2158 = t605 * t2157;
    let t2159 = t144 * t2158;
    let t2164 = 4.0_f64 / 27.0_f64 * t89 * t1637 * t161;
    let t2165 = t1882 * t576;
    let t2167 = t1882 * t611;
    let t2170 = t574 * t616 * t558;
    (t2158, t2159, t2164, t2165, t2167, t2170)
}
