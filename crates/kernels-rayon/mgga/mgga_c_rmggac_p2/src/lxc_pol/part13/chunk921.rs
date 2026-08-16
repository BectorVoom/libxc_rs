//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 921/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk921(t40278: f64, t7478: f64, t14243: f64, t16503: f64, t552: f64, t7482: f64, t14237: f64, t559: f64, t7461: f64, t7467: f64, t2298: f64, t26490: f64) -> (f64, f64, f64, f64, f64) {
    let t40279 = t40278 * t7478;
    let t40283 = t16503 * t14243 * t552 * t7482;
    let t40287 = t16503 * t14237 * t559 * t7461;
    let t40291 = t16503 * t14243 * t559 * t7467;
    let t40295 = t26490 * t2298;
    (t40279, t40283, t40287, t40291, t40295)
}
