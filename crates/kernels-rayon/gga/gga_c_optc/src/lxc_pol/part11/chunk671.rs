//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 671/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk671(t39: f64, t6363: f64, t1765: f64, t745: f64, t1764: f64, t518: f64, t622: f64, t517: f64, t11: f64, t2: f64, t1776: f64, t525: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6364 = t6363 * t39;
    let t6366 = t1765 * t745;
    let t6367 = t1764 * t6366;
    let t6369 = t518 * t622;
    let t6370 = t517 * t6369;
    let t6373 = 1.0_f64/pow_3_2(t11);
    let t6374 = t6373 * t2;
    let t6375 = t6374 * t39;
    let t6377 = t1776 * t6366;
    let t6379 = t525 * t6369;
    (t6364, t6367, t6370, t6374, t6375, t6377, t6379)
}
