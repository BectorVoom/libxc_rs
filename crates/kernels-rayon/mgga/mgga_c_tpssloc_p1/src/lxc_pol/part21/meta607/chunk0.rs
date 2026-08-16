//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2369/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2369(t11985: f64, t526: f64, t11998: f64, t528: f64, t2405: f64, t2419: f64, t690: f64, t703: f64) -> (f64, f64, f64) {
    let t39419 = 1.0_f64 / t526 / t11985;
    let t39436 = 1.0_f64 / t528 / t11998;
    let t39463 = 0.4274e0_f64 * t690 * t2419 * t2405 * t703;
    (t39419, t39436, t39463)
}
