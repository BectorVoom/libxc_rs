//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 622/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk622<F: Float>(t390: F, t391: F, t387: F, t5039: F, t68: F, t70: F) -> (F, F, F, F) {
    let t5141 = t390 * t390;
    let t5143 = F::cast_from(1.0_f64) / t391 / t5141;
    let t5144 = t387 * t5143;
    let t5150 = F::cast_from(0.505765839233979_f64) * t5039;
    let t5153 = t68 * t70;
    (t5141, t5144, t5150, t5153)
}
