//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1156/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1156(t17311: f64, t8186: f64, t5897: f64, t8207: f64, t2253: f64, t7271: f64, t12345: f64, t2069: f64, t4189: f64, t7397: f64, t6028: f64, t6927: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t29421 = 4.0_f64 * t17311 * t8186;
    let t29423 = 2.0_f64 * t5897 * t8207;
    let t29424 = t2253 * t7271;
    let t29426 = 6.0_f64 * t12345 * t29424;
    let t29427 = t8207 * t2069;
    let t29429 = 4.0_f64 * t4189 * t29427;
    let t29430 = t2253 * t7397;
    let t29432 = 2.0_f64 * t4189 * t29430;
    let t29433 = t6028 * t6927;
    (t29421, t29423, t29424, t29426, t29427, t29429, t29430, t29432, t29433)
}
