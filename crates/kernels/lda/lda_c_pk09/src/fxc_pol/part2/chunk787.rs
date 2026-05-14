//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 787/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk787<F: Float>(t1098: F, t9150: F, t1052: F, t8141: F, t1011: F, t1041: F, t106: F, t2341: F, t2363: F, t2380: F, t3138: F, t3142: F, t4429: F, t4438: F, t4445: F, t4449: F, t4451: F, t7896: F, t8726: F, t8821: F, t9159: F) -> (F,) {
    let t9422 = t1098 * t9150;
    let t9438 = t1052 * t8141;
    let t9440 = t2363 * t3138 / 6.0 + t2363 * t3142 / 6.0 + t9422 / 6.0 + t4429 / 6.0 - t4438 / 6.0 - t4445 / 6.0 - 0.20475546210383508 * t7896 - 0.14975624337724558 * t8726 + t4449 / 9.0 - 0.14975624337724558 * t8821 + t1041 * t2341 / 6.0 - t2380 * t1011 / 6.0 - t106 * t9159 / 6.0 + t4451 / 9.0 - t9438 / 9.0;
    (t9440,)
}
