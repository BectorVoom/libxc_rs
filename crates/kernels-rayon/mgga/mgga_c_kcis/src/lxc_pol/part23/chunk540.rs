//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 540/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk540(t1602: f64, t4425: f64, t1599: f64, t1611: f64, t25: f64, t286: f64, t3977: f64, t3754: f64, t617: f64, t2642: f64, t491: f64, t610: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4426 = t4425 * t1602;
    let t4427 = t1599 * t4426;
    let t4429 = t25 * t1611;
    let t4430 = t1599 * t4429;
    let t4432 = t286 * t3977;
    let t4433 = t617 * t3754;
    let t4434 = t4433 * t2642;
    let t4435 = t4432 * t4434;
    let t4438 = t610 * t491;
    (t4426, t4427, t4429, t4430, t4432, t4434, t4435, t4438)
}
