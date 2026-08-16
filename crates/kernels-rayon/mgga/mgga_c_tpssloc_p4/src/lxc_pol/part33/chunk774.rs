//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 774/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk774(t25: f64, t1409: f64, t1965: f64, t40: f64, t7552: f64, t7643: f64, t1484: f64, t28: f64, t1915: f64, t1530: f64, t1649: f64, t1877: f64, t2522: f64, t6670: f64, t7541: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64, f64, f64) {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t7648 = piecewise3(t115, t7552, t1965 * t1409 / 2.0_f64 + t7643 * t40 / 2.0_f64);
    let t7649 = t28 * t1484;
    let t7650 = t1915 * t7649;
    let t7656 = t28 * t1530;
    let t7663 = 3.0_f64 / 2.0_f64 * t2522 * t7650 + t1877 * t7541 * t28 / 2.0_f64 - t1877 * t6670 * t7656 / 2.0_f64 + t1877 * t1915 * t1649 / 2.0_f64;
    (t7648, t7649, t7656, t7663)
}
