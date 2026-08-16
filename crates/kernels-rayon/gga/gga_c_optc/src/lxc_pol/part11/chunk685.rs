//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 685/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk685(t102: f64, t6599: f64, t108: f64, t176: f64, t203: f64, t1864: f64, t587: f64, t6407: f64, t601: f64, t6424: f64, t6427: f64, t580: f64, t6419: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6600 = t6599 * t102;
    let t6602 = t176 * t6600 * t108;
    let t6604 = t6602 * t203 / 2.0_f64;
    let t6617 = t1864 * t6407 * t587;
    let t6619 = 0.35089340384731224426e1_f64 * t601 * t6617;
    let t6636 = t6424 * t6407 * t6427;
    let t6638 = 0.1025389702100779493e4_f64 * t601 * t6636;
    let t6642 = t580 * t6419 * t587;
    (t6602, t6604, t6617, t6619, t6636, t6638, t6642)
}
