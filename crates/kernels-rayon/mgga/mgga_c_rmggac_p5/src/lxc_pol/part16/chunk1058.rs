//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1058/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1058(t36639: f64, t9713: f64, t2057: f64, t31273: f64, t2868: f64, t8413: f64, t46058: f64, t739: f64, t39103: f64, t9222: f64, t40323: f64, t40313: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t48011 = t36639 * t9713;
    let t48014 = t31273 * t2057;
    let t48017 = t2868 * t8413;
    let t48022 = t739 * t46058;
    let t48027 = t9222 * t39103;
    let t48029 = t9222 * t40323;
    let t48031 = t9222 * t40313;
    (t48011, t48014, t48017, t48022, t48027, t48029, t48031)
}
