//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 973/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk973(t3507: f64, t4229: f64, t4232: f64, t13394: f64, t6322: f64, t6321: f64, t1516: f64, t4181: f64, t493: f64, t14234: f64, t4204: f64, t6331: f64) -> (f64, f64, f64, f64) {
    let t14344 = t3507 * t4229;
    let t14345 = t14344 * t4232;
    let t14347 = t6322 * t13394;
    let t14348 = t6321 * t14347;
    let t14350 = t4181 * t1516;
    let t14351 = t493 * t14350;
    let t14353 = t4204 * t14234;
    let t14354 = t6331 * t14353;
    (t14345, t14348, t14351, t14354)
}
