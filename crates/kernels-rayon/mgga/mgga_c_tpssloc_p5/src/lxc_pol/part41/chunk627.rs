//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 627/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk627(t1317: f64, t3726: f64, t2566: f64, t535: f64, t795: f64, t154: f64, t557: f64) -> (f64, f64, f64) {
    let t3727 = t3726 * t1317;
    let t3731 = 0.26388888888888888888e-2_f64 * t2566 * t535 * t795;
    let t3732 = t154 * t557;
    (t3727, t3731, t3732)
}
