//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 518/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk518<F: Float>(t3086: F, t61: F, t96: F, t839: F, t62: F, t891: F, t917: F, t127: F, t567: F, t126: F) -> (F, F, F, F, F, F) {
    let t3088 = t96 * t61 * t3086;
    let t3089 = t839 * t3088;
    let t3090 = F::new(22.07984838129906) * t3089;
    let t3100 = t891 * t62 * t3086;
    let t3101 = t917 * t3100;
    let t3102 = F::new(1.800081713982063) * t3101;
    let t3103 = t127 * t567;
    let t3104 = t126 * t3103;
    (t3089, t3090, t3101, t3102, t3103, t3104)
}
