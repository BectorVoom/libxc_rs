//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 980/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk980(t11135: f64, t10292: f64, t281: f64, t415: f64, t1114: f64, t2403: f64, t3298: f64, t699: f64, t3301: f64, t3304: f64, t241: f64, t3439: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11195 = 0.93011851851851851854e0_f64 * t11135;
    let t11203 = t281 * t10292 * t415;
    let t11204 = 0.36514074074074074075e0_f64 * t11203;
    let t11211 = t2403 * t1114;
    let t11213 = t699 * t3298;
    let t11215 = t699 * t3301;
    let t11217 = t699 * t3304;
    let t11219 = t241 * t3439;
    (t11195, t11203, t11204, t11211, t11213, t11215, t11217, t11219)
}
