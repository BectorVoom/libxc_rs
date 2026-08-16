//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 397/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk397<F: Float>(t10: F, t1542: F, t296: F, t2336: F, t793: F, t89: F, t375: F, t825: F, t2347: F, t295: F, t683: F, t798: F) -> (F, F, F, F, F, F, F, F) {
    let t2652 = t10 * t1542 * t296;
    let t2653 = F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t2652;
    let t2655 = t89 * t2336 * t793;
    let t2656 = t2655 / F::cast_from(27.0_f64);
    let t2658 = t89 * t375 * t825;
    let t2659 = t2658 / F::cast_from(9.0_f64);
    let t2660 = t295 * t2347;
    let t2665 = t683 * t798;
    (t2652, t2653, t2655, t2656, t2658, t2659, t2660, t2665)
}
