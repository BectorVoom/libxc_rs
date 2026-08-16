//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 803/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk803(t3263: f64, t883: f64, t3449: f64, t972: f64, t2712: f64, t3096: f64, t3430: f64, t1044: f64, t640: f64, t916: f64, t128: f64, t6: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9375 = t3263 * t883;
    let t9378 = t3449 * t972;
    let t9383 = t3096 * t2712;
    let t9384 = t3430 * t9383;
    let t9386 = t640 * t1044;
    let t9387 = t916 * t9386;
    let t9388 = t6 * t128;
    (t9375, t9378, t9384, t9386, t9387, t9388)
}
