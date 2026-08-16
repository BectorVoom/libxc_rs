//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1275/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1275(t1699: f64, t8202: f64, t339: f64, t5550: f64, t790: f64, t64: f64, t8275: f64, t2376: f64, t785: f64, t17954: f64, t789: f64, t17942: f64, t223: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t61024 = t1699 * t8202;
    let t61033 = t339 * t5550 * t790;
    let t61038 = t8275 * t64;
    let t61050 = t339 * t5550 * t2376;
    let t61051 = t61050 * t785;
    let t61057 = t339 * t17954 * t789;
    let t61062 = t17942 * t223;
    (t61024, t61033, t61038, t61050, t61051, t61057, t61062)
}
