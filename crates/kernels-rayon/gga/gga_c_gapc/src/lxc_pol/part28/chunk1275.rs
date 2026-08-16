//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1275/1429 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1275(t3691: f64, t9160: f64, t11362: f64, t28169: f64, t35112: f64, t5218: f64, t1044: f64, t515: f64, t169: f64, t19: f64, t3665: f64, t116: f64, t1882: f64, t9092: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t35188 = t3691 * t9160;
    let t35190 = t11362 * t28169;
    let t35192 = t35112 * t5218;
    let t35194 = t515 * t1044;
    let t35197 = t169 * t35194 * t19 * t3665;
    let t35200 = t116 * t1882 * t9092;
    (t35188, t35190, t35192, t35194, t35197, t35200)
}
