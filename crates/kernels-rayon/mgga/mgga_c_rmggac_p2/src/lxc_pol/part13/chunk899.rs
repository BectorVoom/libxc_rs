//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 899/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk899(t39892: f64, t7411: f64, t7288: f64, t8659: f64, t2286: f64, t7921: f64, t14249: f64, t16503: f64, t559: f64, t7482: f64, t16504: f64, t2318: f64, t34975: f64, t7467: f64) -> (f64, f64, f64, f64, f64) {
    let t39893 = t39892 * t7411;
    let t39899 = t8659 * t7288;
    let t39901 = t7921 * t2286;
    let t39907 = t16503 * t14249 * t559 * t7482;
    let t39911 = t34975 * t16504 * t2318 * t7467;
    (t39893, t39899, t39901, t39907, t39911)
}
