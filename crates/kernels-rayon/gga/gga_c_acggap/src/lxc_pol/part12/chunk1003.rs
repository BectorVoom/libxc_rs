//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1003/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1003(t1089: f64, t3201: f64, t598: f64, t8484: f64, t8489: f64, t1980: f64, t7458: f64, t525: f64, t922: f64, t1181: f64, t30282: f64, t599: f64) -> (f64, f64, f64, f64, f64) {
    let t33898 = t598 * t1089 * t3201 * t8484;
    let t33901 = t3201 * t8489;
    let t33903 = t1980 * t7458 * t33901;
    let t33911 = t525 * t922;
    let t33914 = t30282 * t1181 * t599 * t33911;
    (t33898, t33901, t33903, t33911, t33914)
}
