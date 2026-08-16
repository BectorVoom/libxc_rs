//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 987/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk987(t1140: f64, t4434: f64, t1101: f64, t360: f64, t1181: f64, t3361: f64, t540: f64, t1165: f64, t12816: f64, t4417: f64, t322: f64, t368: f64, t384: f64, t398: f64, t4875: f64) -> (f64, f64, f64, f64, f64) {
    let t16319 = t1140 * t4434;
    let t16325 = t1101 * t360;
    let t16328 = t3361 * t1181 * t540 * t16325;
    let t16332 = t3361 * t1165 * t4417 * t12816;
    let t16356 = t384 * t398 * t368 * t4875 * t322;
    (t16319, t16325, t16328, t16332, t16356)
}
