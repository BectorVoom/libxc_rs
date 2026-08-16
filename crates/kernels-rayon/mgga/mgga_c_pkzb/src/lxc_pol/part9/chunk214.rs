//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 214/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk214(t218: f64, t220: f64, t675: f64, t208: f64, t655: f64, t219: f64, t657: f64, t668: f64, t670: f64, t673: f64) -> (f64, f64, f64, f64, f64) {
    let t677 = t218 * t675 * t220;
    let t678 = 0.82156666666666666667e-1_f64 * t677;
    let t679 = t208 * t655;
    let t681 = t218 * t219 * t679;
    let t683 = 0.1898925e1_f64 * t668 - t670 + 0.8969e0_f64 * t657 + 0.3071625e0_f64 * t673 - t678 + 0.24647e0_f64 * t681;
    (t677, t678, t679, t681, t683)
}
