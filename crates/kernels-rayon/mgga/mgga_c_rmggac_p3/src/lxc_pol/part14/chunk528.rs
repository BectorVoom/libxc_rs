//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 528/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk528(t2069: f64, t333: f64, t262: f64, t7198: f64, t2144: f64, t22: f64, t507: f64) -> (f64, f64, f64, f64, f64) {
    let t7199 = t2069 * t333;
    let t7200 = t262 * t7199;
    let t7201 = t7198 * t7200;
    let t7202 = 0.81823984962736025184e-1_f64 * t7201;
    let t7203 = t2144 * t22;
    let t7204 = t507 * t7203;
    (t7199, t7200, t7202, t7203, t7204)
}
