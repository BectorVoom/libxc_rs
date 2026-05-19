//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 642/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk642<F: Float>(t5039: F, t5161: F, t5045: F, t5190: F, t5208: F, t5212: F, t5068: F, t373: F, t4762: F, t332: F, t383: F, t4767: F) -> (F, F, F, F, F, F, F, F, F) {
    let t5516 = F::cast_from(0.04525483399593904_f64) * t5039;
    let t5520 = F::cast_from(0.30249879055454143_f64) * t5161;
    let t5529 = F::cast_from(0.03016988933062603_f64) * t5045;
    let t5530 = F::cast_from(0.025208232546211785_f64) * t5190;
    let t5535 = F::cast_from(0.22687409291590604_f64) * t5208;
    let t5536 = F::cast_from(0.22687409291590604_f64) * t5212;
    let t5538 = F::cast_from(0.010056629776875343_f64) * t5068;
    let t5542 = t4762 * t373;
    let t5544 = F::cast_from(0.018289183791044262_f64) * t332 * t5542;
    let t5546 = F::cast_from(8.282336896725763_f64) * t383 * t4767;
    (t5516, t5520, t5529, t5530, t5535, t5536, t5538, t5544, t5546)
}
