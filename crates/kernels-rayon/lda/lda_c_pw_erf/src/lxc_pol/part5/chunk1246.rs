//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1246/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1246(t22360: f64, t18158: f64, t18163: f64, t15582: f64, t2193: f64, t1446: f64, t7475: f64, t13925: f64, t15108: f64, t15109: f64, t15111: f64, t22350: f64, t22352: f64, t22354: f64, t22358: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t22361 = 8.0_f64 / 45.0_f64 * t22360;
    let t22362 = 16.0_f64 / 15.0_f64 * t18158;
    let t22363 = 8.0_f64 / 15.0_f64 * t18163;
    let t22367 = 4.0_f64 / 5.0_f64 * t15582 * t2193;
    let t22369 = 8.0_f64 / 15.0_f64 * t1446 * t7475;
    let t22370 = t22350 + t22352 + t22354 + t22358 + t22361 - t13925 - t22362 - t22363 - t15108 - 2.0_f64 / 3.0_f64 * t15109 + 8.0_f64 / 27.0_f64 * t15111 - t22367 + t22369;
    (t22361, t22362, t22363, t22367, t22369, t22370)
}
