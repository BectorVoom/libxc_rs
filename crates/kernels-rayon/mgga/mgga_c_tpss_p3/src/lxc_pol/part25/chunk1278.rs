//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1278/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1278(t61870: f64, t640: f64, t2073: f64, t599: f64, t68: f64, t7594: f64, t582: f64, t7690: f64, t18646: f64, t5483: f64, t1675: f64, t18645: f64, t5506: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t61871 = t61870 * t640;
    let t61873 = t599 * t2073;
    let t61877 = t68 * t7594;
    let t62019 = t7690 * t582;
    let t62259 = t5483 * t18646;
    let t62262 = t1675 * t18645 * t5506;
    (t61871, t61873, t61877, t62019, t62259, t62262)
}
