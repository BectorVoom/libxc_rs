//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 497/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk497(t154: f64, t2559: f64, t222: f64, t2563: f64, t805: f64, t119: f64, t2379: f64, t210: f64, t2553: f64, t225: f64, t2591: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2600 = t2559 * t154;
    let t2602 = 35.0_f64 / 432.0_f64 * t2600 * t222;
    let t2603 = t2563 * t805;
    let t2605 = t119 * t2379;
    let t2606 = t210 * t2605;
    let t2610 = t210 * t119 * t2553;
    let t2613 = t2591 * t225;
    (t2600, t2602, t2603, t2606, t2610, t2613)
}
