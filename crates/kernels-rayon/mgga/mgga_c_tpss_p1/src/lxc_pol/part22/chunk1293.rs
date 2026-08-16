//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1293/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1293(t339: f64, t5550: f64, t790: f64, t2179: f64, t64: f64, t8275: f64, t2376: f64, t785: f64, t17960: f64, t2372: f64, t17954: f64, t789: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t61033 = t339 * t5550 * t790;
    let t61034 = t61033 * t2179;
    let t61038 = t8275 * t64;
    let t61050 = t339 * t5550 * t2376;
    let t61051 = t61050 * t785;
    let t61054 = t17960 * t2372;
    let t61057 = t339 * t17954 * t789;
    (t61033, t61034, t61038, t61050, t61051, t61054, t61057)
}
