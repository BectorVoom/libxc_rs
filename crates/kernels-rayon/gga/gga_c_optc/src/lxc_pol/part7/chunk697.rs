//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 697/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk697(t102: f64, t6599: f64, t108: f64, t176: f64, t203: f64, t2226: f64, t616: f64, t2234: f64, t758: f64, t1986: f64, t1990: f64, t6342: f64, t6356: f64, t6526: f64, t6530: f64, t6563: f64, t6571: f64, t6572: f64) -> (f64, f64, f64, f64) {
    let t6600 = t6599 * t102;
    let t6602 = t176 * t6600 * t108;
    let t6604 = t6602 * t203 / 2.0_f64;
    let t6605 = t2226 * t616;
    let t6607 = t176 * t6605 * t108;
    let t6608 = t6607 * t203;
    let t6610 = t2234 * t758;
    let t6612 = t1986 * t1990;
    let t6613 = 0.35089340384731224426e1_f64 * t6612;
    let t6614 = t6342 + t6526 - t6356 + 3.0_f64 / 2.0_f64 * t6530 + t6563 * t203 / 2.0_f64 - t6571 + 35.0_f64 / 3.0_f64 * t6572 + t6604 + 3.0_f64 / 2.0_f64 * t6608 + 3.0_f64 * t6610 + t6613;
    (t6602, t6607, t6613, t6614)
}
