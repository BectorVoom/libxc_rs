//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 542/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk542<F: Float>(t54: F, t623: F, t48: F, t633: F, t3223: F, t810: F, t3290: F, t664: F, t673: F, t662: F) -> (F, F, F, F, F) {
    let t3344 = t623 * t54;
    let t3348 = t48 * t633;
    let t3368 = t810 * t3223;
    let t3371 = F::cast_from(19.489173774580152_f64) * t810 * t3290;
    let t3383 = t673 * t664 * t623;
    let t3384 = t662 * t3383;
    (t3344, t3348, t3368, t3371, t3384)
}
