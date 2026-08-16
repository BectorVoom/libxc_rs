//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 745/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk745(t12656: f64, t7428: f64, t7427: f64, t969: f64, t825: f64, t3209: f64, t7290: f64, t2365: f64, t6111: f64, t10037: f64, t7785: f64, t12651: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12657 = t7428 * t12656;
    let t12658 = t7427 * t12657;
    let t12660 = t969 * t12656;
    let t12661 = t825 * t12660;
    let t12663 = t7290 * t3209;
    let t12664 = t2365 * t12663;
    let t12665 = t6111 * t12664;
    let t12667 = t10037 * t7785;
    let t12669 = t969 * t12651;
    (t12657, t12658, t12660, t12661, t12663, t12664, t12665, t12667, t12669)
}
