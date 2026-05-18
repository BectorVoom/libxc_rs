//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 178/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk178<F: Float>(t612: F, t15: F, t48: F, t75: F, t71: F) -> (F, F, F, F) {
    let t613 = F::new(0.9421211958699838) * t612;
    let t614 = t15 * t48;
    let t615 = t614 * t75;
    let t616 = t71 * t615;
    (t613, t614, t615, t616)
}
