//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 1155/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk1155(t16471: f64, t2553: f64, t30324: f64, t3402: f64, t519: f64, t6: f64, t1084: f64, t11927: f64, t1461: f64, t291: f64, t8709: f64, t1971: f64, t818: f64, t8448: f64, t9846: f64) -> (f64, f64, f64) {
    let t34264 = t3402 * t519 * t16471 * t2553 * t6 * t30324;
    let t34269 = t1084 * t1461 * t8709 * t291 * t11927;
    let t34274 = t1084 * t1971 * t8448 * t818 * t9846;
    (t34264, t34269, t34274)
}
