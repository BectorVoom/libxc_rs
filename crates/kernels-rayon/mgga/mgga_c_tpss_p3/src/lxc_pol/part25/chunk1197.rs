//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1197/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1197(t1268: f64, t1659: f64, t1333: f64, t18394: f64, t640: f64, t18397: f64, t3532: f64, t5527: f64, t1270: f64, t4397: f64, t1206: f64, t197: f64, t507: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t19581 = t1659 * t1268;
    let t19588 = t18394 * t1333;
    let t19590 = t1333 * t640;
    let t19591 = t18397 * t19590;
    let t19593 = t5527 * t3532;
    let t19604 = t1270 * t4397;
    let t19609 = t1659 * t1206;
    let t19619 = t197 * t507;
    (t19581, t19588, t19590, t19591, t19593, t19604, t19609, t19619)
}
