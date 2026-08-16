//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 717/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk717(t5296: f64, t618: f64, t144: f64, t174: f64, t46: f64, t123: f64, t475: f64, t574: f64, t550: f64, t1667: f64, t1670: f64, t1497: f64, t1613: f64, t542: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5297 = t5296 * t618;
    let t5304 = 1.0_f64 / t174 / t144;
    let t5305 = t5304 * t46;
    let t5322 = t475 * t574 * t123;
    let t5324 = 0.56968947174242584612e-3_f64 * t550 * t5322;
    let t5325 = t1670 * t1667;
    let t5331 = t1613 * t1497 * t542;
    (t5297, t5304, t5305, t5322, t5324, t5325, t5331)
}
