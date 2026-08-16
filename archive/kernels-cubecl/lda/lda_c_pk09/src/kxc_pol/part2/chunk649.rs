//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 649/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk649<F: Float>(t1487: F, t5308: F, t1301: F, t1315: F, t1402: F, t5031: F, t1287: F, t365: F, t1342: F, t5081: F, t1280: F, t1435: F) -> (F, F, F, F, F, F, F) {
    let t5643 = t1487 * t5308;
    let t5646 = t1301 * t1301;
    let t5647 = F::cast_from(1.0_f64) / t5646;
    let t5654 = t1315 * t5308;
    let t5658 = t1402 * t5031;
    let t5659 = t5658 * t1287;
    let t5664 = t365 * t5031;
    let t5670 = t1342 * t5081;
    let t5672 = t1280 * t1435;
    (t5643, t5647, t5654, t5659, t5664, t5670, t5672)
}
