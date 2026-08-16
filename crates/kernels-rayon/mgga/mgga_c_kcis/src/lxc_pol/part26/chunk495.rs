//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 495/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk495(t1602: f64, t4425: f64, t1599: f64, t1611: f64, t25: f64, t286: f64, t3977: f64, t3754: f64, t617: f64, t491: f64, t610: f64, t990: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4426 = t4425 * t1602;
    let t4427 = t1599 * t4426;
    let t4429 = t25 * t1611;
    let t4430 = t1599 * t4429;
    let t4432 = t286 * t3977;
    let t4433 = t617 * t3754;
    let t4438 = t610 * t491;
    let t4439 = t4438 * t990;
    (t4426, t4427, t4429, t4430, t4432, t4433, t4439)
}
