//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1367/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1367(t28: f64, t265: f64, t504: f64, t106667: f64, t106716: f64, t106606: f64, t1409: f64, t1972: f64, t20217: f64, t28803: f64, t52: f64, t5398: f64, t7664: f64, t1441: f64, t5493: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> (f64, f64) {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t106717 = t106667 + t106716;
    let t106718 = piecewise3(t505, 0.0_f64, t106606);
    let t106728 = piecewise3(t401, t106717, t106718 * t52 / 2.0_f64 - 3.0_f64 / 2.0_f64 * t28803 * t1409 - 3.0_f64 / 2.0_f64 * t7664 * t5398 - t1972 * t20217 / 2.0_f64);
    let t106731 = t1441 * t5493;
    (t106728, t106731)
}
