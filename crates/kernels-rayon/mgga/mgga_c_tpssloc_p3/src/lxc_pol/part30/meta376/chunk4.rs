//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1433/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1433(t40: f64, t52: f64, t16549: f64, t16554: f64, t16558: f64, t3966: f64, t4080: f64, t607: f64, t73: f64, t5392: f64, t9438: f64, t2440: f64, t5398: f64, t4087: f64, t76: f64, zeta_threshold: f64) -> (f64, f64) {
    let t146 = t40 <= zeta_threshold;
    let t150 = t52 <= zeta_threshold;
    let t16562 = piecewise3(t146, 0.0_f64, -8.0_f64 / 27.0_f64 * t16549 * t607 + 8.0_f64 / 9.0_f64 * t4080 * t3966 + 4.0_f64 / 9.0_f64 * t16554 * t607 + 4.0_f64 / 3.0_f64 * t73 * t16558);
    let t16563 = t9438 * t5392;
    let t16568 = t2440 * t5398;
    let t16574 = piecewise3(t150, 0.0_f64, 8.0_f64 / 27.0_f64 * t16563 * t607 + 8.0_f64 / 9.0_f64 * t4087 * t3966 + 4.0_f64 / 9.0_f64 * t16568 * t607 - 4.0_f64 / 3.0_f64 * t76 * t16558);
    (t16562, t16574)
}
