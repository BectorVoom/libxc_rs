//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 967/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk967(t41209: f64, t8764: f64, t5207: f64, t649: f64, t5211: f64, t7599: f64, t5199: f64, t5187: f64, t5218: f64, t5194: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t41210 = t8764 * t41209;
    let t41212 = t649 * t5207;
    let t41213 = t8764 * t41212;
    let t41215 = t649 * t5211;
    let t41216 = t7599 * t41215;
    let t41218 = t649 * t5199;
    let t41219 = t7599 * t41218;
    let t41221 = t649 * t5187;
    let t41222 = t7599 * t41221;
    let t41224 = t649 * t5218;
    let t41225 = t7599 * t41224;
    let t41227 = t649 * t5194;
    (t41210, t41212, t41213, t41215, t41216, t41218, t41219, t41221, t41222, t41224, t41225, t41227)
}
