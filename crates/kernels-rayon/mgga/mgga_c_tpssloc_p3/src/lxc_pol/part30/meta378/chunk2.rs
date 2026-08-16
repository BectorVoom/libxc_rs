//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1439/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1439(t52: f64, t5392: f64, t638: f64, t5398: f64, t78: f64, t16558: f64, t3966: f64, t4111: f64, t607: f64, t771: f64, t16648: f64, zeta_threshold: f64) -> f64 {
    let t150 = t52 <= zeta_threshold;
    let t16649 = t638 * t5392;
    let t16654 = t78 * t5398;
    let t16660 = piecewise3(t150, 0.0_f64, -8.0_f64 / 27.0_f64 * t16649 * t607 - 4.0_f64 / 9.0_f64 * t4111 * t3966 - 2.0_f64 / 9.0_f64 * t16654 * t607 - 2.0_f64 / 3.0_f64 * t771 * t16558);
    let t16662 = t16648 / 2.0_f64 + t16660 / 2.0_f64;
    t16662
}
