//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 226/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk226(t19: f64, t598: f64, t83: f64, t85: f64, t24: f64, t583: f64, t61: f64) -> (f64, f64, f64, f64) {
    let t600 = 0.1356e2_f64 * t19 * t598;
    let t604 = 1.0_f64 / t85 / t83;
    let t605 = t24 * t604;
    let t625 = 1.0_f64 / t61 / t583;
    (t600, t604, t605, t625)
}
