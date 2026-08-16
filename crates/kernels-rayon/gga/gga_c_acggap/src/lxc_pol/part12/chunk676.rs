//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 676/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk676(t1165: f64, t3176: f64, t7351: f64, t2068: f64, t3196: f64, t604: f64, t7337: f64, t1181: f64, t3169: f64, t4210: f64, t7346: f64, t589: f64, t968: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7353 = t1165 * t7351 * t3176;
    let t7354 = t2068 * t7353;
    let t7357 = t1165 * t604 * t3196;
    let t7358 = t7337 * t7357;
    let t7361 = t1181 * t604 * t3169;
    let t7362 = t2068 * t7361;
    let t7365 = t1165 * t604 * t4210;
    let t7366 = t7346 * t7365;
    let t7368 = t589 * t968;
    (t7353, t7354, t7357, t7358, t7361, t7362, t7365, t7366, t7368)
}
