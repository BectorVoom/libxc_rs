//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 793/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk793(t40: f64, t52: f64, t5392: f64, t5398: f64, t75: f64, t767: f64, t771: f64, t78: f64, zeta_threshold: f64) -> f64 {
    let t146 = t40 <= zeta_threshold;
    let t150 = t52 <= zeta_threshold;
    let t5536 = piecewise3(t146, 0.0_f64, -2.0_f64 / 9.0_f64 * t75 * t5392 + 2.0_f64 / 3.0_f64 * t767 * t5398);
    let t5542 = piecewise3(t150, 0.0_f64, -2.0_f64 / 9.0_f64 * t78 * t5392 - 2.0_f64 / 3.0_f64 * t771 * t5398);
    let t5544 = t5536 / 2.0_f64 + t5542 / 2.0_f64;
    t5544
}
