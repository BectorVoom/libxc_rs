//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 780/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk780(t1165: f64, t1421: f64, t604: f64, t7493: f64, t3220: f64, t56: f64, t2065: f64, t2450: f64) -> (f64, f64, f64, f64, f64) {
    let t8458 = t1165 * t604 * t1421;
    let t8459 = t7493 * t8458;
    let t8461 = t56 * t3220;
    let t8462 = t2065 * t8461;
    let t8463 = t2450 * t8462;
    (t8458, t8459, t8461, t8462, t8463)
}
