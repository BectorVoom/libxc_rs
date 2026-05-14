//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 442/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk442<F: Float>(t1496: F, t1498: F, t1500: F, t1502: F, t2502: F, t2505: F, t2542: F, t2546: F) -> (F,) {
    let t2594 = t1496 - 0.22687409291590604 * t2542 + t1498 + 0.22687409291590604 * t2546 + t1500 - 0.04525483399593904 * t2502 + t1502 + 0.04525483399593904 * t2505;
    (t2594,)
}
