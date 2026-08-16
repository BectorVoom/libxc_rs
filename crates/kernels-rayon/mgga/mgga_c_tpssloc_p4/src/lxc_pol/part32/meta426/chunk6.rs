//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1651/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1651(t25: f64, t6320: f64, t67: f64, t758: f64, t12061: f64, t6305: f64, t3664: f64, t5397: f64, t16557: f64, t2219: f64, t5134: f64, t514: f64, t606: f64, zeta_threshold: f64) -> (f64, f64) {
    let t26 = t25 <= zeta_threshold;
    let t19541 = t6320 * t67;
    let t19542 = t19541 * t758;
    let t19543 = 0.18311447306006545054e-3_f64 * t19542;
    let t19547 = t12061 * t6305;
    let t19552 = t3664 * t5397;
    let t19558 = piecewise3(t26, 0.0_f64, -8.0_f64 / 27.0_f64 * t19547 * t606 + 16.0_f64 / 9.0_f64 * t5134 * t2219 + 4.0_f64 / 9.0_f64 * t19552 * t606 + 4.0_f64 / 3.0_f64 * t514 * t16557);
    (t19543, t19558)
}
