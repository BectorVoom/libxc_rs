//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 962/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk962(t11203: f64, t1113: f64, t11163: f64, t136: f64, t11172: f64, t1114: f64, t2403: f64, t3298: f64, t699: f64, t3301: f64, t3304: f64, t241: f64, t3439: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11204 = 0.36514074074074074075e0_f64 * t11203;
    let t11205 = t1113 * t11163;
    let t11206 = t136 * t11205;
    let t11208 = t1113 * t11172;
    let t11209 = t136 * t11208;
    let t11211 = t2403 * t1114;
    let t11213 = t699 * t3298;
    let t11215 = t699 * t3301;
    let t11217 = t699 * t3304;
    let t11219 = t241 * t3439;
    (t11204, t11206, t11209, t11211, t11213, t11215, t11217, t11219)
}
