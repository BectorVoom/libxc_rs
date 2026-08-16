//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1004/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1004(t12743: f64, t1545: f64, t13183: f64, t1541: f64, t13850: f64, t2450: f64) -> (f64, f64, f64) {
    let t16867 = t12743 * t1545;
    let t16869 = t13183 * t1541;
    let t16871 = t2450 * t13850;
    (t16867, t16869, t16871)
}
