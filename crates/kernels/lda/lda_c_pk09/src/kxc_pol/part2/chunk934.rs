//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 934/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk934<F: Float>(t1216: F, t2513: F, t332: F, t9770: F, t10: F, t5420: F, t2517: F, t1513: F, t9851: F, t1494: F, t2611: F, t382: F, t9739: F) -> (F, F, F, F, F, F) {
    let t9885 = t1216 * t2513;
    let t9887 = t332 * t9770;
    let t9889 = t5420 * t10;
    let t9890 = t9889 * t2517;
    let t9892 = t1513 * t9851;
    let t9894 = t1494 * t2611;
    let t9896 = t382 * t9739;
    (t9885, t9887, t9890, t9892, t9894, t9896)
}
