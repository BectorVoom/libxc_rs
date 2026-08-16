//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1007/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1007(t12747: f64, t1470: f64, t1140: f64, t4840: f64, t1165: f64, t3809: f64, t4282: f64, t530: f64, t3346: f64, t4384: f64, t4396: f64, t310: f64, t4197: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16940 = t12747 * t1470;
    let t16942 = t1140 * t4840;
    let t16946 = t4282 * t1165 * t530 * t3809;
    let t16950 = t4282 * t1165 * t530 * t3346;
    let t16980 = t4396 * t4384;
    let t16986 = t310 * t4197;
    (t16940, t16942, t16946, t16950, t16980, t16986)
}
