//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 1056/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk1056(t126: f64, t932: f64, t1038: f64, t11925: f64, t16826: f64, t19: f64, t7877: f64, t15615: f64, t3327: f64, t17713: f64, t23466: f64, t8676: f64) -> (f64, f64, f64, f64, f64) {
    let t30153 = t932 * t126;
    let t30158 = t11925 * t1038 * t7877 * t19 * t16826;
    let t30167 = t3327 * t15615;
    let t30187 = t3327 * t17713;
    let t30288 = t8676 * t23466;
    (t30153, t30158, t30167, t30187, t30288)
}
