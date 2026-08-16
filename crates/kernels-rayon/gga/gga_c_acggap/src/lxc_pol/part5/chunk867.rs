//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 867/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk867(t1240: f64, t3101: f64, t381: f64, t1032: f64, t3811: f64, t151: f64, t3668: f64, t940: f64, t947: f64, t3765: f64, t932: f64, t1077: f64, t435: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12419 = t381 * t1240 * t3101;
    let t12421 = t1032 * t3811;
    let t12457 = t151 * t940 * t3668;
    let t12458 = t12457 * t947;
    let t12460 = t3765 * t932;
    let t12473 = t435 * t1077;
    (t12419, t12421, t12457, t12458, t12460, t12473)
}
