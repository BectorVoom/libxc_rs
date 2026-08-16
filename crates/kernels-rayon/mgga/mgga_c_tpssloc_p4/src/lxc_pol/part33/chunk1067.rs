//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1067/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1067(t22430: f64, t3: f64, t1458: f64, t5456: f64, t5493: f64, t1401: f64, t16524: f64, t20162: f64, t20347: f64, t3941: f64, t5371: f64, t576: f64, t577: f64) -> (f64, f64, f64, f64) {
    let t22431 = t3 * t22430;
    let t22445 = t5456 * t1458;
    let t22448 = t1458 * t5493;
    let t22453 = 0.45e1_f64 * t22430 * t577 + 0.405e2_f64 * t20162 * t1458 + 81.0_f64 * t16524 * t5456 + 0.405e2_f64 * t5371 * t5493 + 27.0_f64 * t576 * t22445 + 81.0_f64 * t3941 * t22448 + 0.135e2_f64 * t1401 * t20347;
    (t22431, t22445, t22448, t22453)
}
