//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 939/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk939(t31404: f64, t7507: f64, t7513: f64, t174: f64, t30779: f64, t7322: f64, t3125: f64, t721: f64, t7447: f64, t7819: f64, t1981: f64, t2015: f64) -> (f64, f64, f64, f64, f64) {
    let t31406 = t7507 * t31404 * t7513;
    let t31407 = 0.94322839859753421338e-2_f64 * t31406;
    let t31419 = t7322 * t30779 * t174;
    let t31421 = t31419 * t3125 * t721;
    let t31426 = t7447 * t7819;
    let t31428 = t2015 * t1981;
    (t31407, t31419, t31421, t31426, t31428)
}
