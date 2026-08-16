//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1182/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1182(t147: f64, t1669: f64, t99: f64, t12329: f64, t12337: f64, t12354: f64, t12356: f64, t12358: f64, t13558: f64, t13561: f64, t13570: f64, t13574: f64, t15324: f64, t15326: f64, t15329: f64, t15332: f64) -> (f64, f64) {
    let t15548 = t99 * t1669 * t147;
    let t15563 = 1.1517333333333333_f64 * t15326 - 0.31992592592592595_f64 * t15329 + 0.10666666666666667_f64 * t15548 * t13574 * t15324 - 1.7276_f64 * t15332 - 0.32_f64 * t15548 * t13570 * t15324 + 0.03950617283950617_f64 * t13558 - 0.2725925925925926_f64 * t13561 - 1.135737037037037_f64 * t12329 + 0.07464938271604939_f64 * t12337 + 0.06398518518518519_f64 * t12354 - 0.047988888888888886_f64 * t12356 - 0.19195555555555555_f64 * t12358;
    (t15548, t15563)
}
