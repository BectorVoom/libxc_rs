//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 926/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk926<F: Float>(t2149: F, t4007: F, t633: F, t93: F, t11092: F, t1853: F, t1672: F, t2847: F, t7102: F, t451: F, t476: F, t2795: F, t7308: F, t2042: F, t7286: F, t7296: F) -> (F, F, F, F, F, F, F, F) {
    let t11489 = t4007 * t2149;
    let t11490 = t11489 * t633;
    let t11491 = t93 * t11490;
    let t11494 = t1853 * t11092;
    let t11500 = t2847 * t1672;
    let t11502 = t7102 * t11092;
    let t11504 = t451 * t476;
    let t11509 = t2795 * t7308;
    let t11510 = t11509 * t2042;
    let t11512 = t2795 * t7286;
    let t11515 = t7296 * t11092;
    (t11491, t11494, t11500, t11502, t11504, t11510, t11512, t11515)
}
