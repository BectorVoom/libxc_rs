//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 794/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk794<F: Float>(t7611: F, t7658: F, t7686: F, t7714: F, t7736: F, t7771: F, t7970: F, t7997: F, t8045: F, t8076: F, t8088: F, t8112: F, t8144: F, t8257: F, t8347: F, t8385: F, t8419: F, t8448: F, t8473: F, t8514: F, t8548: F, t8575: F, t8599: F, t8631: F, t8654: F, t8677: F, t8730: F, t8757: F, t8848: F, t8890: F, t8972: F, t9554: F) -> (F,) {
    let t9559 = t8575 + t7611 + t7736 + t8548 + t9554 + t8848 + t8677 + t8257 + t7970 + t8088 + t7658 + t8890 + t8144 + t7686 + t8385 + t8419 + t8112 + t8631 + t8599 + t7997 + t8045 + t7714 + t8473 + t8654 + t8972 + t8757 + t8448 + t8347 + t8730 + t8514 + t7771 + t8076;
    (t9559,)
}
