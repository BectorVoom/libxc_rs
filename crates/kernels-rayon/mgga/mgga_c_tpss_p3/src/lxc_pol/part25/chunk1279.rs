//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1279/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1279(t31455: f64, t5784: f64, t18669: f64, t7682: f64, t1675: f64, t1679: f64, t72: f64, t789: f64, t1981: f64, t234: f64, t38: f64, t5489: f64) -> (f64, f64, f64, f64, f64) {
    let t62277 = t31455 * t5784;
    let t62280 = t7682 * t18669;
    let t62294 = 1232.0_f64 / 81.0_f64 * t1675 * t789 * t72 * t1679;
    let t62306 = t1981 * t38 * t234;
    let t62307 = t62306 * t5489;
    (t62277, t62280, t62294, t62306, t62307)
}
