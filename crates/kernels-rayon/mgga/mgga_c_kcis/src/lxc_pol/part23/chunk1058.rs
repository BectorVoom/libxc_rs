//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1058/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1058(t27544: f64, t4294: f64, t1468: f64, t4298: f64, t1395: f64, t4303: f64, t4307: f64, t27512: f64, t27515: f64, t27518: f64, t27522: f64, t27524: f64, t27527: f64, t27530: f64, t27533: f64, t27535: f64, t27537: f64, t27539: f64, t27541: f64) -> (f64, f64, f64, f64, f64) {
    let t27545 = t27544 * t4294;
    let t27547 = t1468 * t4298;
    let t27549 = t1395 * t4303;
    let t27551 = t1395 * t4307;
    let t27553 = t27512 / 16.0_f64 - t27515 / 8.0_f64 + t27518 / 12.0_f64 + t27522 / 8.0_f64 - t27524 / 12.0_f64 - t27527 / 16.0_f64 - t27530 / 72.0_f64 + t27533 / 24.0_f64 - t27535 / 128.0_f64 + t27537 / 64.0_f64 - t27539 / 48.0_f64 - t27541 / 64.0_f64 + t27545 / 48.0_f64 + t27547 / 128.0_f64 - t27549 / 288.0_f64 - t27551 / 96.0_f64;
    (t27545, t27547, t27549, t27551, t27553)
}
