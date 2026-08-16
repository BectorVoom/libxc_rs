//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 798/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk798(t38872: f64, t7487: f64, t8466: f64, t35207: f64, t8469: f64, t1591: f64, t2046: f64, t2050: f64, t31: f64, t1657: f64, t638: f64, t7292: f64, t8486: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t38873 = 0.10248087766267884742e-3_f64 * t38872;
    let t38874 = t7487 * t8466;
    let t38876 = t35207 * t8469;
    let t38881 = t2046 * t2050 * t1591 * t31;
    let t38882 = 0.43368970657079495312e-4_f64 * t38881;
    let t38886 = t2046 * t2050 * t1657 * t31;
    let t38887 = 0.43368970657079495312e-4_f64 * t38886;
    let t38889 = t638 * t7292 * t8486;
    (t38873, t38874, t38876, t38882, t38887, t38889)
}
