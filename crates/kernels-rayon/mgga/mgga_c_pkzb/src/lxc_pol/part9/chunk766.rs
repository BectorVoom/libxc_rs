//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 766/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk766(t5290: f64, t5406: f64, t158: f64, t625: f64, t1791: f64, t633: f64, t1790: f64, t1812: f64, t183: f64, t5373: f64, t1719: f64, t621: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5407 = t5290 + t5406;
    let t5408 = t5407 * t158;
    let t5417 = t625 * t625;
    let t5418 = 1.0_f64 / t5417;
    let t5419 = t1791 * t633;
    let t5420 = t5418 * t5419;
    let t5423 = t1790 * t633;
    let t5424 = t5423 * t1812;
    let t5427 = t183 * t5373;
    let t5431 = t621 * t1719;
    (t5407, t5408, t5417, t5418, t5419, t5420, t5424, t5427, t5431)
}
