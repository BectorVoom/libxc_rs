//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 939/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk939(t1154: f64, t330: f64, t348: f64, t1758: f64, t3251: f64, t1114: f64, t13786: f64, t345: f64, t2952: f64, t4601: f64, t4600: f64, t313: f64, t4625: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t14269 = t1154 * t348 * t330;
    let t14272 = t3251 * t1758;
    let t14274 = t1114 * t13786;
    let t14275 = t345 * t14274;
    let t14278 = t4601 * t2952;
    let t14279 = t4600 * t14278;
    let t14282 = t313 * t4625;
    (t14269, t14272, t14274, t14275, t14278, t14279, t14282)
}
