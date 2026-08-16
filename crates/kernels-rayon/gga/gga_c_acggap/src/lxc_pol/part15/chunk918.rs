//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 918/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk918(t2450: f64, t7560: f64, t3401: f64, t7559: f64, t1170: f64, t3378: f64, t7336: f64) -> (f64, f64, f64, f64) {
    let t31346 = t2450 * t7560;
    let t31349 = t7559 * t3401;
    let t31350 = t1170 * t31349;
    let t31362 = t3378 * t7336;
    (t31346, t31349, t31350, t31362)
}
