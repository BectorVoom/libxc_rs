//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 626/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk626<F: Float>(t5166: F, t281: F, t226: F, t1248: F, t1249: F, t1253: F) -> (F, F, F) {
    let t5167 = F::new(8.0) * t5166;
    let t5168 = t281 * t281;
    let t5169 = F::new(1.0) / t5168;
    let t5170 = t226 * t5169;
    let t5177 = t1248 * t1253 * t1249;
    (t5167, t5170, t5177)
}
