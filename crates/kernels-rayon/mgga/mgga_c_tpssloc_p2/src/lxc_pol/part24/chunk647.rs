//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 647/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk647(t3590: f64, t466: f64, t1236: f64, t225: f64, t1239: f64, t496: f64, t68: f64, t1251: f64, t1243: f64, t3534: f64, t3032: f64, t3502: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3591 = t466 * t3590;
    let t3593 = t1236 * t225;
    let t3597 = 1.0_f64 / t1239 / t496;
    let t3598 = t68 * t3597;
    let t3599 = t1251 * t1251;
    let t3600 = t3598 * t3599;
    let t3604 = t3534 * t1243;
    let t3609 = t3032 * t3502;
    (t3591, t3593, t3598, t3599, t3600, t3604, t3609)
}
