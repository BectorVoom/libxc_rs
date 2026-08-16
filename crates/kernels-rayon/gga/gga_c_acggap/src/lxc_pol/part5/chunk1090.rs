//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1090/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1090(t14947: f64, t1674: f64, t19441: f64, t19444: f64, t19451: f64, t19452: f64, t19453: f64, t19454: f64, t19455: f64, t19456: f64, t19510: f64, t291: f64, t301: f64, t6610: f64, t6621: f64, t839: f64, t96: f64) -> f64 {
    let t19514 = 12.0_f64 * t1674 * t19444 * t301 + 6.0_f64 * t1674 * t6610 * t839 + 3.0_f64 * t19510 * t291 * t96 + 12.0_f64 * t14947 * t6621 - t19441 - t19451 + t19452 + t19453 + t19454 - t19455 + t19456;
    t19514
}
