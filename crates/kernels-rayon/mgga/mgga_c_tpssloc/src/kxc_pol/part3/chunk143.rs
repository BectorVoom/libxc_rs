//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 143/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk143(t25: f64, t28: f64, t265: f64, t396: f64, t40: f64, t52: f64, dens_threshold: f64, rho0: f64, rho1: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t26 = t25 <= zeta_threshold;
    let t29 = t28 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t399 = piecewise3(t115, t265 * t25 / 2.0_f64, t396 * t40 / 2.0_f64);
    let t401 = rho1 <= dens_threshold || t29;
    let t404 = 1.0_f64 / t52;
    let t405 = pow_1_3(t404);
    (t399, t404, t405)
}
