//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1017/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1017(t1008: f64, t4759: f64, t1005: f64, t4720: f64, t4754: f64, t1032: f64, t5089: f64, t4547: f64, t13298: f64, t13364: f64, t16325: f64, t525: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17230 = t1008 * t4759;
    let t17232 = t1005 * t4720;
    let t17234 = t1008 * t4754;
    let t17236 = t1032 * t5089;
    let t17238 = t1008 * t4547;
    let t17254 = t13298 * t13364 * t525 * t16325;
    (t17230, t17232, t17234, t17236, t17238, t17254)
}
