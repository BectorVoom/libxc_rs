//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 503/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk503(t420: f64, t56: f64, t137: f64, t495: f64, t506: f64, t6: f64, t119: f64, t182: f64) -> (f64, f64, f64, f64) {
    let t2066 = t56 * t420;
    let t2297 = t137 * t495;
    let t2325 = t6 * t506;
    let t2450 = t119 * t182;
    (t2066, t2297, t2325, t2450)
}
