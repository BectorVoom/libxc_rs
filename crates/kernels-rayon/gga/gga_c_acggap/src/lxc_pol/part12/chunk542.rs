//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 542/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk542(t1165: f64, t3253: f64, t3457: f64, t3456: f64, t1172: f64, t1530: f64, t396: f64, t980: f64, t409: f64, t932: f64, t935: f64, t322: f64, t922: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3459 = t1165 * t3253 * t3457;
    let t3460 = t3456 * t3459;
    let t3462 = t1530 * t1172;
    let t3476 = t980 * t396;
    let t3477 = t3476 * t409;
    let t3479 = t935 * t932;
    let t3491 = t922 * t322;
    (t3459, t3460, t3462, t3476, t3477, t3479, t3491)
}
