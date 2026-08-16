//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 709/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk709(t12693: f64, t12706: f64, t12223: f64, t2562: f64, t883: f64, t943: f64, t2558: f64, t3732: f64, t12405: f64, t12784: f64, t13288: f64, t13291: f64, t13292: f64, t13293: f64, t13294: f64, t13295: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13898 = 0.63904876589867916128e-1_f64 * t12693;
    let t13899 = 0.63904876589867916128e-1_f64 * t12706;
    let t13934 = t2562 * t883 * t12223;
    let t13935 = t943 * t13934;
    let t13937 = t3732 * t2558;
    let t13938 = t943 * t13937;
    let t14266 = t13288 + 2.0_f64 * t12784 - 2.0_f64 * t12405 - t13291 - t13292 + t13293 + t13294 + t13295;
    (t13898, t13899, t13934, t13935, t13937, t13938, t14266)
}
