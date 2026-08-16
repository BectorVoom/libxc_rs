//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 754/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk754(t1223: f64, t1966: f64, t1968: f64, t464: f64, t1973: f64, t214: f64, t4517: f64, t2189: f64, t4443: f64, t674: f64, t638: f64, t7292: f64, t7301: f64) -> (f64, f64, f64, f64, f64) {
    let t35326 = t1966 * t464 * t1223 * t1968;
    let t35327 = t35326 * t1973;
    let t35331 = t1966 * t4517 * t214 * t1968;
    let t35470 = t2189 * t4443 * t674;
    let t35478 = t638 * t7292 * t7301;
    (t35326, t35327, t35331, t35470, t35478)
}
