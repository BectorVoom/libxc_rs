//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 931/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk931(t1089: f64, t12610: f64, t2079: f64, t2080: f64, t1967: f64, t7767: f64, t1459: f64, t1980: f64, t31024: f64, t7458: f64, t2117: f64, t980: f64) -> (f64, f64, f64, f64) {
    let t31245 = t2079 * t1089 * t12610 * t2080;
    let t31247 = t1967 * t7767;
    let t31251 = t1980 * t7458 * t1459 * t31024;
    let t31253 = t980 * t2117;
    (t31245, t31247, t31251, t31253)
}
