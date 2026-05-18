//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 836/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk836<F: Float>(t8128: F, t917: F, t8334: F, t890: F, t7608: F, t839: F, t2254: F, t4023: F, t623: F, t8318: F, t8322: F, t8330: F) -> (F, F, F, F, F, F, F, F) {
    let t8517 = t917 * t8128;
    let t8519 = t890 * t8334;
    let t8521 = t839 * t7608;
    let t8524 = t4023 * t2254 * t623;
    let t8525 = t890 * t8524;
    let t8527 = t890 * t8318;
    let t8529 = t890 * t8322;
    let t8531 = t917 * t8330;
    (t8517, t8519, t8521, t8524, t8525, t8527, t8529, t8531)
}
