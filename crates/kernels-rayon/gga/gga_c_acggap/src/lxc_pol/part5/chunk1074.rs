//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1074/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1074(t381: f64, t4225: f64, t879: f64, t1648: f64, t3243: f64, t1160: f64, t1539: f64, t18906: f64, t377: f64, t5310: f64, t1652: f64, t980: f64) -> (f64, f64, f64, f64, f64) {
    let t19112 = t381 * t4225 * t879;
    let t19117 = t3243 * t1648;
    let t19122 = t1160 * t18906 * t1539;
    let t19129 = t377 * t5310;
    let t19133 = t980 * t1652;
    (t19112, t19117, t19122, t19129, t19133)
}
