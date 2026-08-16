//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 584/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk584(t3476: f64, t409: f64, t932: f64, t935: f64, t322: f64, t922: f64, t1426: f64, t175: f64, t384: f64, t1137: f64, t962: f64, t1131: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3477 = t3476 * t409;
    let t3479 = t935 * t932;
    let t3491 = t922 * t322;
    let t3493 = t1426 * t175 * t3491;
    let t3494 = t384 * t3493;
    let t3504 = t1137 * t962;
    let t3529 = t1131 * t322;
    (t3477, t3479, t3491, t3493, t3494, t3504, t3529)
}
