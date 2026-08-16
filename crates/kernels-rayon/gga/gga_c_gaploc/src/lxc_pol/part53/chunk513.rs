//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 513/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk513(t188: f64, t9189: f64, t3085: f64, t4130: f64, t590: f64, t1339: f64, t3116: f64, t3196: f64, t7014: f64, t2488: f64, t9278: f64, t2487: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9351 = t188 * t9189;
    let t9354 = t4130 * t3085;
    let t9355 = t9354 * t590;
    let t9358 = t1339 * t3116;
    let t9359 = t9358 * t590;
    let t9362 = t7014 * t3196;
    let t9363 = 0.38342925953920749676e0_f64 * t9362;
    let t9364 = t2488 * t9278;
    let t9365 = t2487 * t9364;
    (t9351, t9354, t9355, t9359, t9362, t9363, t9365)
}
