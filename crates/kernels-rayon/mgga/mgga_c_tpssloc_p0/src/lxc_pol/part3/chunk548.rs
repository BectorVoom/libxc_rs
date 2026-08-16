//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 548/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk548(t40: f64, t52: f64, t2535: f64, t761: f64, t718: f64, t751: f64, t2244: f64, t2250: f64, t75: f64, t767: f64, t771: f64, t78: f64, zeta_threshold: f64) -> (f64, f64, f64, f64) {
    let t146 = t40 <= zeta_threshold;
    let t150 = t52 <= zeta_threshold;
    let t2537 = 0.5848223622634646207e0_f64 * t761 * t2535;
    let t2538 = t718 * t751;
    let t2539 = 2.0_f64 * t2538;
    let t2545 = piecewise3(t146, 0.0_f64, -2.0_f64 / 9.0_f64 * t75 * t2244 + 2.0_f64 / 3.0_f64 * t767 * t2250);
    let t2551 = piecewise3(t150, 0.0_f64, -2.0_f64 / 9.0_f64 * t78 * t2244 - 2.0_f64 / 3.0_f64 * t771 * t2250);
    let t2553 = t2545 / 2.0_f64 + t2551 / 2.0_f64;
    (t2537, t2538, t2539, t2553)
}
