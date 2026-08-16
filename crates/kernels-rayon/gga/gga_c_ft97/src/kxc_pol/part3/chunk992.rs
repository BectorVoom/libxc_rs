//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 992/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk992(t19362: f64, t2862: f64, t319: f64, t5299: f64, t875: f64, t840: f64, t871: f64, t15147: f64, t1901: f64, t19318: f64, t19320: f64, t19322: f64, t19326: f64, t19330: f64, t19335: f64, t19340: f64, t19343: f64, t19346: f64, t19351: f64, t19355: f64, t19359: f64, t446: f64) -> f64 {
    let t19364 = t2862 * t319 * t19362;
    let t19367 = t5299 * t875;
    let t19369 = t840 * t871 * t19367;
    let t19372 = -2.0_f64 / 9.0_f64 * t19318 + 2.0_f64 / 81.0_f64 * t19320 + t19322 / 27.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t19326 - 2.0_f64 / 3.0_f64 * t446 * t19330 - t446 * t19335 / 3.0_f64 + t1901 * t19340 / 9.0_f64 + 2.0_f64 / 27.0_f64 * t19343 - 2.0_f64 / 3.0_f64 * t446 * t19346 - 8.0_f64 / 27.0_f64 * t15147 + 2.0_f64 / 9.0_f64 * t446 * t19351 + 4.0_f64 / 3.0_f64 * t446 * t19355 + 2.0_f64 / 3.0_f64 * t446 * t19359 + 2.0_f64 / 3.0_f64 * t446 * t19364 + t446 * t19369 / 3.0_f64;
    t19372
}
