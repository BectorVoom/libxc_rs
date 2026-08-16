//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 933/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk933(t11428: f64, t1461: f64, t1030: f64, t8716: f64, t129: f64, t5541: f64, t3021: f64, t5544: f64, t5462: f64, t5549: f64, t11387: f64, t1649: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let pi = (M_PI as f64);
    let t11429 = t11428 * pi;
    let t11430 = t1461 * t11429;
    let t11431 = t1030 * t11430;
    let t11432 = t11431 * t8716;
    let t11434 = t5541 * t129;
    let t11435 = t3021 * t5544;
    let t11436 = t11434 * t11435;
    let t11438 = t5462 * t129;
    let t11439 = t3021 * t5549;
    let t11440 = t11438 * t11439;
    let t11442 = t11387 * t1649;
    (t11430, t11431, t11432, t11434, t11435, t11436, t11438, t11439, t11440, t11442)
}
