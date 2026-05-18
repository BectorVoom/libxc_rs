//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 638/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk638<F: Float>(t1287: F, t5391: F, t1487: F, t4979: F, t1481: F, t4982: F, t332: F, t5081: F, t1525: F, t5294: F, t1435: F, t1543: F) -> (F, F, F, F, F, F, F) {
    let t5392 = t5391 * t1287;
    let t5395 = F::new(2.427516195194328) * t1487 * t4979;
    let t5396 = t1481 * t4982;
    let t5404 = t332 * t5081;
    let t5408 = t1525 * t5294;
    let t5409 = F::new(5.40024514194619) * t5408;
    let t5414 = t1543 * t1435;
    (t5392, t5395, t5396, t5404, t5408, t5409, t5414)
}
