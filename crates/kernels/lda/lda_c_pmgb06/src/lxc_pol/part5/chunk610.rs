//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 610/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk610<F: Float>(t395: F, t2799: F, t247: F, t902: F, t2142: F, t686: F, t248: F, t2158: F, t643: F, t3912: F, t760: F, t1: F, t1068: F) -> (F, F, F, F, F, F, F, F) {
    let t4461 = F::new(4.0) * t395;
    let t4462 = F::new(12.0) * t2799;
    let t4472 = t247 * t902;
    let t4481 = t2142 * t686;
    let t4483 = F::new(2.0) * t248 * t4481;
    let t4485 = F::new(8.0) * t643 * t2158;
    let t4486 = t3912 * t760;
    let t4489 = t1068 * t1;
    (t4461, t4462, t4472, t4481, t4483, t4485, t4486, t4489)
}
