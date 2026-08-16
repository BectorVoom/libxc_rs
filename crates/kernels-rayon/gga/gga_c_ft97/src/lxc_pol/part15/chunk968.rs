//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 968/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk968(t21748: f64, t8392: f64, t1882: f64, t21492: f64, t21719: f64, t681: f64, t89: f64, t21761: f64, t21674: f64, t21754: f64, t21537: f64, t21416: f64, t258: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t80345 = t8392 * t21748;
    let t80399 = t1882 * t21492;
    let t80406 = t89 * t681 * t21719;
    let t80412 = t8392 * t21761;
    let t80429 = t8392 * t21674;
    let t80431 = t8392 * t21754;
    let t80433 = t1882 * t21537;
    let t80460 = t258 * t21416;
    (t80345, t80399, t80406, t80412, t80429, t80431, t80433, t80460)
}
