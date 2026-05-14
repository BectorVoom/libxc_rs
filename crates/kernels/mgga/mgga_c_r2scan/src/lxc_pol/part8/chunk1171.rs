//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1171/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1171<F: Float>(t234: F, t5260: F, t5299: F, t21380: F, t597: F, t1860: F, t5276: F, t732: F, t5280: F, t160: F, t164: F, t5869: F, t604: F, t5876: F, t601: F, t161: F, t6077: F) -> (F, F, F, F, F, F, F, F, F) {
    let t22319 = 0.24934837112181307812e4 * t234 * t5260 * t5299;
    let t22320 = t597 * t21380;
    let t22321 = t1860 * t22320;
    let t22325 = t732 * t5276;
    let t22329 = t732 * t5280;
    let t22335 = 11880.0 * t160 * t164;
    let t22336 = t5869 * t604;
    let t22340 = t601 * t5876;
    let t22344 = 32760.0 * t161 / t6077;
    (t22319, t22320, t22321, t22325, t22329, t22335, t22336, t22340, t22344)
}
