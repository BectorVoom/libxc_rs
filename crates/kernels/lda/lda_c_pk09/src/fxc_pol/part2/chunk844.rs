//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 844/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk844<F: Float>(t5043: F, t5056: F, t5158: F, t5166: F, t5177: F, t5193: F, t5956: F, t5972: F, t9623: F, t9631: F, t9635: F, t9742: F, t9750: F, t9948: F, t9952: F, t9956: F, t9959: F) -> (F,) {
    let t10098 = 0.7661514025603425 * t9948 + 0.7661514025603425 * t9952 - 0.7661514025603425 * t9956 + 0.510767601706895 * t9959 - 0.15282509383508946 * t9623 - 0.05094169794502982 * t9631 - 0.15282509383508946 * t9635 - 0.15282509383508946 * t9742 - 0.15282509383508946 * t9750 - 0.15282509383508946 * t5043 - 0.05094169794502982 * t5056 + t5956 - 0.510767601706895 * t5177 + 0.510767601706895 * t5193 + t5972 - 1.532302805120685 * t5158 + 1.532302805120685 * t5166;
    (t10098,)
}
