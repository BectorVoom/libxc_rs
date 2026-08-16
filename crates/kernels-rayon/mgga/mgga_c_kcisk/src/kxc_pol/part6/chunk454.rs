//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 454/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk454(t3532: f64, t459: f64, t1337: f64, t306: f64, t1422: f64, t425: f64, t1390: f64, t1173: f64, t416: f64, t298: f64, t301: f64, t446: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3533 = t459 * t3532;
    let t3538 = t1337 * t306;
    let t3539 = t3538 * t459;
    let t3544 = t1422 * t425;
    let t3549 = t459 * t1390;
    let t3558 = t1173 * t459;
    let t3564 = t416 * t306;
    let t3571 = t298 * t446 * t301;
    (t3533, t3539, t3544, t3549, t3558, t3564, t3571)
}
