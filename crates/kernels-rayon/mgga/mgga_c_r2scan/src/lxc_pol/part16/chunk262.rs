//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 262/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk262(t106: f64, t795: f64, t797: f64, t97: f64, t292: f64, t415: f64, rho0: f64, tau0: f64) -> (f64, f64, f64) {
    let t799 = t97 * t106 * t795 * t797;
    let t800 = rho0 * rho0;
    let t802 = 1.0_f64 / t292 / t800;
    let t803 = tau0 * t802;
    let t806 = t415 / 2.0_f64;
    (t799, t803, t806)
}
