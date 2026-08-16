//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 182/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk182(t19: f64, t598: f64, t582: f64, t586: f64, t589: f64, t593: f64, t596: f64, t83: f64, t85: f64, t24: f64) -> (f64, f64, f64, f64) {
    let t600 = 0.1356e2_f64 * t19 * t598;
    let t601 = t582 - t586 + t589 - t593 + t596 - t600;
    let t604 = 1.0_f64 / t85 / t83;
    let t605 = t24 * t604;
    (t600, t601, t604, t605)
}
