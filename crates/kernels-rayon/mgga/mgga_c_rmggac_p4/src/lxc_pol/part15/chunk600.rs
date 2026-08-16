//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 600/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk600(t7756: f64, t8465: f64, t2010: f64, t2415: f64, t7760: f64, t7349: f64, t270: f64, t575: f64, t2039: f64, t638: f64, t31: f64, t2046: f64, t2050: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8466 = t8465 * t7756;
    let t8467 = t2010 * t8466;
    let t8469 = t2415 * t7760;
    let t8470 = t7349 * t8469;
    let t8475 = t575 * t270;
    let t8477 = t638 * t2039 * t8475;
    let t8482 = t575 * t31;
    let t8484 = t2046 * t2050 * t8482;
    (t8466, t8467, t8469, t8470, t8475, t8477, t8482, t8484)
}
