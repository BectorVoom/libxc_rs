//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 602/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk602<F: Float>(t2460: F, t375: F, t89: F, t194: F, t196: F, t122: F, t2427: F, t677: F, t2380: F, t2382: F, t2379: F, t191: F, t2999: F, t26: F, t1771: F, t685: F) -> (F, F, F, F, F, F, F, F, F) {
    let t9520 = t89 * t375 * t2460;
    let t9523 = 1.0 / t196 / t194;
    let t9524 = t122 * t9523;
    let t9533 = t677 * t2427;
    let t9542 = t2380 * t2382;
    let t9543 = t2379 * t9542;
    let t9555 = t2999 * t191;
    let t9556 = t26 * t9555;
    let t9557 = 28.0 / 27.0 * t9556;
    let t9558 = t1771 * t685;
    (t9520, t9524, t9533, t9542, t9543, t9555, t9556, t9557, t9558)
}
