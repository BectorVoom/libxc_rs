//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 903/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk903(t1005: f64, t3531: f64, t3348: f64, t1165: f64, t3451: f64, t4210: f64, t991: f64, t1163: f64, t955: f64, t315: f64, t4197: f64, t1162: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13539 = t1005 * t3531;
    let t13545 = t1005 * t3348;
    let t13573 = t3451 * t1165 * t991 * t4210;
    let t13582 = t1163 * t1165 * t991 * t955;
    let t13584 = t315 * t4197;
    let t13585 = t13584 * t1162;
    (t13539, t13545, t13573, t13582, t13584, t13585)
}
