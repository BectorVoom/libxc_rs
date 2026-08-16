//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2801/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2801(t4119: f64, t4255: f64, t41008: f64, t5550: f64, t16783: f64, t41196: f64, t118: f64, t16662: f64, t2576: f64, t794: f64, t16787: f64, t2563: f64) -> (f64, f64, f64, f64, f64) {
    let t59198 = t4255 * t4119;
    let t59204 = t41008 * t5550;
    let t59206 = t41196 * t16783;
    let t59214 = t2576 * t118 * t794 * t16662;
    let t59216 = t2563 * t16787;
    (t59198, t59204, t59206, t59214, t59216)
}
