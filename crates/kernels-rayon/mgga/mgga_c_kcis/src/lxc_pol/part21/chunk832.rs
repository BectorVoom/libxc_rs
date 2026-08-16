//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 832/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk832(t3393: f64, t3416: f64, t3402: f64, t1157: f64, t752: f64, t3407: f64, t3166: f64, t330: f64, t3412: f64, t1160: f64, t318: f64, t86: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10552 = t3393 * t3416;
    let t10554 = t3393 * t3402;
    let t10556 = t752 * t1157;
    let t10558 = t3393 * t3407;
    let t10594 = t3166 * t330;
    let t10599 = t3393 * t3412;
    let t10631 = t86 * t318 * t1160;
    (t10552, t10554, t10556, t10558, t10594, t10599, t10631)
}
