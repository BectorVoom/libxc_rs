//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 717/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk717(t1022: f64, t20526: f64, t4649: f64, t4719: f64, t20022: f64, t9025: f64, t7761: f64, t89: f64, t1964: f64, t356: f64, t20039: f64, t569: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t20527 = t20526 * t1022;
    let t20529 = t4649 * t4719;
    let t20534 = t9025 * t20022;
    let t20536 = t89 * t7761 * t20534;
    let t20538 = t1964 * t20022;
    let t20540 = t89 * t356 * t20538;
    let t20542 = t569 * t20039;
    (t20527, t20529, t20534, t20536, t20538, t20540, t20542)
}
