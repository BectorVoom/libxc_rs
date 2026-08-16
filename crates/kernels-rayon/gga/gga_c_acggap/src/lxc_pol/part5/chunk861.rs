//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 861/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk861(t1264: f64, t316: f64, t449: f64, t879: f64, t3034: f64, t309: f64, t180: f64, t3923: f64, t3035: f64, t441: f64, t3912: f64, t852: f64) -> (f64, f64, f64, f64, f64) {
    let t12271 = t316 * t449 * t879 * t1264;
    let t12273 = t309 * t3034;
    let t12276 = 0.15805078039045227836e2_f64 * t12273 * t180 * t3923;
    let t12278 = t3035 * t441 * t3923;
    let t12281 = 0.26341796731742046395e1_f64 * t852 * t3912;
    (t12271, t12273, t12276, t12278, t12281)
}
