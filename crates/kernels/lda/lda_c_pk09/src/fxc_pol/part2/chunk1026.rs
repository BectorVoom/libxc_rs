//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 1026/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk1026<F: Float>(t11078: F, t1681: F, t1745: F, t1947: F, t2795: F, t2042: F, t2871: F, t1240: F, t2751: F, t333: F) -> (F, F, F, F) {
    let t11079 = t1681 * t11078;
    let t11080 = t11079 * t1745;
    let t11083 = t2795 * t1947;
    let t11084 = t11083 * t2042;
    let t11086 = t2871 * t1947;
    let t11087 = t11086 * t2042;
    let t11091 = t2751 * t1240;
    let t11092 = t333 * t11091;
    (t11080, t11084, t11087, t11092)
}
