//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 829/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk829<F: Float>(t3650: F, t4359: F, t1321: F, t374: F, t4044: F, t73: F, t3559: F, t1227: F, t384: F, t1234: F, t4232: F, t1322: F, t4233: F, t123: F, t317: F, t3974: F, t740: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t10544 = t4359 * t3650;
    let t10547 = t1321 * t1321;
    let t10548 = 1.0 / t10547;
    let t10551 = t73 * t4044 * t374;
    let t10558 = t73 * t3559;
    let t10565 = t384 * t1227;
    let t10570 = t4232 * t1234 * t374;
    let t10577 = t1322 * t384;
    let t10578 = t10577 * t4233;
    let t10582 = t4232 * t1227 * t374;
    let t10594 = t123 * t740 * t3974 * t317;
    (t10544, t10548, t10551, t10558, t10565, t10570, t10577, t10578, t10582, t10594)
}
