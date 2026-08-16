//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 1077/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk1077(t40865: f64, t7192: f64, t36920: f64, t7933: f64, t9081: f64, t303: f64, t577: f64, t7934: f64, t357: f64, t132: f64, t1412: f64, t36912: f64, t9082: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t42228 = t7192 * t40865;
    let t42234 = t7933 * t36920 * t9081;
    let t42238 = t7933 * t7934 * t577 * t303;
    let t42239 = 0.72042316457491791906e-3_f64 * t42238;
    let t42242 = t7933 * t7934 * t577 * t357;
    let t42243 = 0.72042316457491791906e-3_f64 * t42242;
    let t42246 = t7933 * t7934 * t1412 * t132;
    let t42247 = 0.72042316457491791906e-3_f64 * t42246;
    let t42248 = t36912 * t9082;
    (t42228, t42234, t42239, t42243, t42247, t42248)
}
