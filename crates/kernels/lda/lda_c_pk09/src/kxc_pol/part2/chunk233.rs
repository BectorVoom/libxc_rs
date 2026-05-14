//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 233/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk233<F: Float>(t721: F, t957: F, t612: F, t616: F, t626: F, t636: F, t653: F, t82: F) -> (F, F, F, F, F, F, F) {
    let t959 = 2.427516195194328 * t957 * t721;
    let t966 = 11.879313099038017 * t612;
    let t967 = 7.919542066025344 * t616;
    let t971 = t966 + t967 + 11.879313099038017 * t626 + 11.879313099038017 * t636 - 11.879313099038017 * t653;
    let t972 = t82 * t82;
    let t973 = t972 + 1.0;
    let t974 = 1.0 / t973;
    let t975 = t971 * t974;
    (t959, t966, t967, t971, t973, t974, t975)
}
