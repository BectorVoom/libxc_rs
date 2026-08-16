//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 662/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk662(t2460: f64, t375: f64, t89: f64, t194: f64, t196: f64, t122: f64, t2427: f64, t677: f64, t2380: f64, t2382: f64, t2379: f64, t191: f64, t2999: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9520 = t89 * t375 * t2460;
    let t9523 = 1.0_f64 / t196 / t194;
    let t9524 = t122 * t9523;
    let t9533 = t677 * t2427;
    let t9542 = t2380 * t2382;
    let t9543 = t2379 * t9542;
    let t9555 = t2999 * t191;
    (t9520, t9524, t9533, t9542, t9543, t9555)
}
