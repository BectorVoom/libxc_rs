//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 444/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk444<F: Float>(t2171: F, t2175: F, t2179: F, t966: F, t967: F, t974: F) -> (F, F) {
    let t2353 = t966 + t967 + F::new(11.879313099038017) * t2171 + F::new(11.879313099038017) * t2175 - F::new(11.879313099038017) * t2179;
    let t2354 = t2353 * t974;
    (t2353, t2354)
}
