//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 532/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk532(t1572: f64, t4350: f64, t1562: f64, t592: f64, t600: f64, t4332: f64, t1341: f64, t1347: f64, t3918: f64, t473: f64, t1356: f64, t3919: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4351 = t4350 * t1572;
    let t4354 = t1562 * t1562;
    let t4355 = 1.0_f64 / t4354;
    let t4356 = t592 * t4355;
    let t4357 = t600 * t600;
    let t4358 = 1.0_f64 / t4357;
    let t4359 = t4332 * t4358;
    let t4363 = t1341 * t1347;
    let t4366 = t473 * t3918;
    let t4367 = t3919 * t1356;
    (t4351, t4354, t4355, t4356, t4357, t4358, t4359, t4363, t4366, t4367)
}
