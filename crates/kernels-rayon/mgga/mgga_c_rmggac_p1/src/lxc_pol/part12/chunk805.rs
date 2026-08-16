//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 805/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk805(t34592: f64, t7441: f64, t7443: f64, t7446: f64, t7452: f64, t7459: f64, t7465: f64, t7471: f64, t7480: f64, t7486: f64, t7488: f64, t8563: f64) -> (f64, f64) {
    let t38251 = -t7441 + t7443 + t7446 + t7452 + t7459 + t7465 - t7471 - t7480 + t7486 + 0.38422568777328955684e-2_f64 * t7488 + t34592;
    let t38254 = 0.27274661654245341728e-1_f64 * t8563;
    (t38251, t38254)
}
