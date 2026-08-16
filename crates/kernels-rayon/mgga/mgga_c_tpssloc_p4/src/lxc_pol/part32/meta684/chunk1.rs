//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2126/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2126(t5445: f64, t641: f64, t72: f64, t19445: f64, t79: f64, t19299: f64, t608: f64, t3966: f64, t2235: f64, t5399: f64, t17635: f64, t605: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t96517 = t72 * t641 * t5445;
    let t96521 = t72 * t79 * t19445;
    let t96535 = t19299 * t608;
    let t96553 = t72 * t79 * t3966;
    let t96556 = t2235 * t5399;
    let t96559 = t605 * t17635;
    (t96517, t96521, t96535, t96553, t96556, t96559)
}
