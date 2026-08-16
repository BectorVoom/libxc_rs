//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1028/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1028(t1864: f64, t1971: f64, t209: f64, t236: f64, t36336: f64, t476: f64, t40231: f64, t9222: f64, t46454: f64, t7192: f64, t2186: f64, t9932: f64) -> (f64, f64, f64, f64) {
    let t47471 = t36336 * t1971 * t236 * t1864 * t476 * t209;
    let t47473 = t9222 * t40231;
    let t47478 = t7192 * t46454;
    let t47484 = t2186 * t9932;
    (t47471, t47473, t47478, t47484)
}
