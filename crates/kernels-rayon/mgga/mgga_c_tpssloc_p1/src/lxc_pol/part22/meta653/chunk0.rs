//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2195/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2195(t16898: f64, t9638: f64, t13258: f64, t16893: f64, t16918: f64, t4191: f64, t46657: f64, t4240: f64, t120: f64, t16752: f64, t16924: f64, t17004: f64, t2563: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t58461 = t9638 * t16898;
    let t58472 = t13258 * t16893;
    let t58474 = t9638 * t16918;
    let t58480 = t46657 * t4191;
    let t58482 = t46657 * t4240;
    let t58495 = t120 * t16752;
    let t58504 = t9638 * t16924;
    let t58528 = t2563 * t17004;
    (t58461, t58472, t58474, t58480, t58482, t58495, t58504, t58528)
}
