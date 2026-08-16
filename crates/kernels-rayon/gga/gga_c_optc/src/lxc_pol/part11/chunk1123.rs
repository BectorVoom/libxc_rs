//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1123/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1123(t3546: f64, t4759: f64, t108: f64, t16287: f64, t176: f64, t203: f64, t729: f64, t16294: f64, t188: f64, t1916: f64, t16300: f64, t16310: f64, t6766: f64) -> (f64, f64, f64, f64, f64) {
    let t48051 = t3546 * t4759;
    let t48058 = t176 * t729 * t16287 * t108 * t203;
    let t48067 = t188 * t1916 * t16294;
    let t48070 = t188 * t1916 * t16300;
    let t48101 = t16310 * t6766;
    (t48051, t48058, t48067, t48070, t48101)
}
