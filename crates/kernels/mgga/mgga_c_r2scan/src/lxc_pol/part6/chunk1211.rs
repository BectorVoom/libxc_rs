//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1211/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1211<F: Float>(t41: F, t4959: F, t725: F, t160: F, t164: F, t5869: F, t604: F, t5876: F, t601: F, t161: F, t6077: F, t18914: F, t230: F, t21519: F, t61: F, t21432: F) -> (F, F, F, F, F, F, F, F) {
    let t22332 = t41 * t4959 * t725;
    let t22335 = 11880.0 * t160 * t164;
    let t22336 = t5869 * t604;
    let t22340 = t601 * t5876;
    let t22344 = 32760.0 * t161 / t6077;
    let t22350 = 840.0 * t18914 * t230;
    let t22352 = 0.57791679765211885293e1 * t61 * t21519;
    let t22354 = 0.3903689268108626343e0 * t61 * t21432;
    (t22332, t22335, t22336, t22340, t22344, t22350, t22352, t22354)
}
