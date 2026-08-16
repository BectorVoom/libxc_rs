//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 777/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk777(t3109: f64, t9120: f64, t1404: f64, t1720: f64, t3108: f64, t5553: f64, t8687: f64, t19: f64, t8768: f64, t611: f64, t3085: f64, t3160: f64, t608: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9121 = t9120 * t3109;
    let t9123 = t1720 * t1404;
    let t9124 = t3108 * t9123;
    let t9126 = t5553 * t8687;
    let t9128 = t8768 * t19;
    let t9129 = t611 * t9128;
    let t9130 = t9129 * t3085;
    let t9132 = t3160 * t608;
    (t9121, t9124, t9126, t9128, t9130, t9132)
}
