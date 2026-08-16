//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 472/1303 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk472(t25: f64, t265: f64, t394: f64, t1070: f64, t1534: f64, t1545: f64, t1559: f64, t1585: f64, t1587: f64, t1591: f64, t1637: f64, t193: f64, t336: f64, t1408: f64, t1409: f64, t396: f64, t40: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64) {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t1642 = piecewise3(t395, t1070 * t1637 * t193 * t336 - t1545 + t1559 + t1585 + t1587 - t1591, t1534);
    let t1647 = piecewise3(t115, t265 * t1408 / 2.0_f64 + t1534 * t25 / 2.0_f64, t396 * t1409 / 2.0_f64 + t1642 * t40 / 2.0_f64);
    (t1642, t1647)
}
