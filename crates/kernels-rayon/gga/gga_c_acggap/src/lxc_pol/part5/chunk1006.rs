//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1006/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1006(t384: f64, t398: f64, t429: f64, t5087: f64, t1487: f64, t368: f64, t879: f64, t3101: f64, t506: f64, t13039: f64, t527: f64, t1140: f64, t5188: f64) -> (f64, f64, f64, f64, f64) {
    let t16916 = t384 * t398 * t429 * t5087;
    let t16921 = t384 * t398 * t368 * t1487 * t879;
    let t16926 = t384 * t398 * t368 * t506 * t3101;
    let t16928 = t13039 * t527;
    let t16930 = t1140 * t5188;
    (t16916, t16921, t16926, t16928, t16930)
}
