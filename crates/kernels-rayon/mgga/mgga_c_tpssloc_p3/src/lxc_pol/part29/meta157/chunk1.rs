//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 835/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk835(t221: f64, t3427: f64, t456: f64, t1176: f64, t135: f64) -> (f64, f64, f64) {
    let t3428 = t221 * t3427;
    let t3430 = 0.18518518518518518518e-3_f64 * t456 * t3428;
    let t3431 = t135 * t1176;
    (t3428, t3430, t3431)
}
