//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 133/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk133(t25: f64, t28: f64, t382: f64, t388: f64, t193: f64, t293: f64, t328: f64, t330: f64, t336: f64, t265: f64, t40: f64, t52: f64, dens_threshold: f64, rho0: f64, rho1: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t26 = t25 <= zeta_threshold;
    let t29 = t28 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t390 = t382 * t388 + 1.0_f64;
    let t391 = f64::ln(t390);
    let t394 = t193 * t336 * t391 - t293 + t328 + t330;
    let t395 = t265 < t394;
    let t396 = piecewise3(t395, t394, t265);
    let t399 = piecewise3(t115, t265 * t25 / 2.0_f64, t396 * t40 / 2.0_f64);
    let t401 = rho1 <= dens_threshold || t29;
    let t404 = 1.0_f64 / t52;
    let t405 = pow_1_3(t404);
    (t390, t396, t399, t404, t405, t394)
}
