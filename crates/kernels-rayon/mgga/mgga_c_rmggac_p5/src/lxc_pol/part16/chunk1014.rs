//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1014/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1014(t10088: f64, t321: f64, t3351: f64, t511: f64, t7248: f64, t333: f64, t7231: f64, t880: f64, t9128: f64, t9765: f64, t2186: f64, t9817: f64) -> (f64, f64, f64, f64) {
    let t47202 = t3351 * t7248 * t511 * t10088 * t321;
    let t47207 = t3351 * t7231 * t880 * t10088 * t333;
    let t47213 = t9128 * t9765;
    let t47215 = t2186 * t9817;
    (t47202, t47207, t47213, t47215)
}
