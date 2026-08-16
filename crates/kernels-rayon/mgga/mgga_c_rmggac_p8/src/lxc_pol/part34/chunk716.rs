//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 716/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk716(t13872: f64, t14363: f64, t13876: f64, t13880: f64, t13884: f64, t14031: f64, t14367: f64, t14035: f64, t1326: f64, t14147: f64, t3057: f64, t14150: f64, t290: f64, t35253: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t70104 = t14363 * t13872;
    let t70106 = t14363 * t13876;
    let t70108 = t14363 * t13880;
    let t70110 = t14363 * t13884;
    let t70123 = t14031 * t14367;
    let t70124 = t70123 * t14035;
    let t70127 = t14147 * t1326 * t3057;
    let t70130 = t70127 * t35253 * t290 * t14150;
    (t70104, t70106, t70108, t70110, t70123, t70124, t70127, t70130)
}
