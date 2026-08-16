//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 740/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk740(t1673: f64, t637: f64, t1675: f64, t191: f64, t1661: f64, t545: f64, t83: f64, t126: f64, t5119: f64, t1545: f64, t546: f64, t513: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5162 = t1673 * t637;
    let t5165 = 1.0_f64 / t1675 / t191;
    let t5169 = t1661 * t545;
    let t5170 = t83 * t5169;
    let t5171 = 3.0_f64 * t5170;
    let t5175 = t5119 * t126;
    let t5176 = t83 * t5175;
    let t5177 = t1545 * t546;
    let t5178 = 36.0_f64 * t5177;
    let t5179 = t1545 * t513;
    (t5162, t5165, t5169, t5170, t5171, t5175, t5176, t5177, t5178, t5179)
}
