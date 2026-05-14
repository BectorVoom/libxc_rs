//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 795/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk795<F: Float>(t44: F, t2: F, t2140: F, t258: F, t620: F, t2605: F, t5647: F, t332: F, t299: F, t94: F, t1299: F, t333: F, t2530: F, t4758: F, t2529: F, t6052: F, zeta_threshold: F) -> (F, F, F, F, F, F, F) {
    let t45 = t44 <= zeta_threshold;
    let t9564 = t44 * t2;
    let t9568 = piecewise3(t45, 0.0, 2.0 * t2140 * t620 + 4.0 * t258 * t9564);
    let t9578 = t2605 * t5647;
    let t9579 = t9578 * t332;
    let t9580 = t94 * t299;
    let t9581 = t9580 * t1299;
    let t9582 = t333 * t9581;
    let t9585 = t2530 * t4758;
    let t9588 = t6052 * t2529;
    (t9568, t9578, t9579, t9581, t9582, t9585, t9588)
}
