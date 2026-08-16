//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 563/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk563(t151: f64, t3328: f64, t947: f64, t377: f64, t941: f64, t322: f64, t839: f64, t1089: f64, t175: f64, t384: f64, t301: f64, t864: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3329 = t151 * t3328;
    let t3330 = t3329 * t947;
    let t3343 = t377 * t941;
    let t3344 = t3343 * t947;
    let t3346 = t839 * t322;
    let t3348 = t1089 * t175 * t3346;
    let t3349 = t384 * t3348;
    let t3355 = t864 * t301;
    (t3329, t3330, t3343, t3344, t3346, t3348, t3349, t3355)
}
