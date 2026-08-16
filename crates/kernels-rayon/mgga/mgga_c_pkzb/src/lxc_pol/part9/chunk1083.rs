//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1083/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1083(t1835: f64, t87: f64, t1971: f64, t5493: f64, t5762: f64, t713: f64, t1908: f64, t1915: f64, t5829: f64, t690: f64, t1731: f64, t218: f64, t220: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17359 = t1835 * t1835;
    let t17361 = 1.0_f64 / t87 / t17359;
    let t17381 = t1971 * t5493;
    let t17385 = t5762 * t713;
    let t17388 = t1908 * t1915;
    let t17391 = t690 * t5829;
    let t17402 = t218 * t1731 * t220;
    (t17361, t17381, t17385, t17388, t17391, t17402)
}
