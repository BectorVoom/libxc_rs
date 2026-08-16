//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1141/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1141(t12329: f64, t12332: f64, t12335: f64, t12337: f64, t12341: f64, t12345: f64, t12348: f64, t12351: f64, t12354: f64, t12356: f64, t12358: f64, t12360: f64, t12362: f64, t12398: f64, t13558: f64, t13561: f64, t13565: f64, t13566: f64, t13570: f64, t13574: f64, t2060: f64, t2969: f64, t473: f64, t9225: f64) -> f64 {
    let t13591 = 0.08_f64 * t2060 * t473 * t2969 + 0.019753086419753086_f64 * t13558 - 0.28444444444444444_f64 * t13561 + 0.023994444444444443_f64 * t9225 - 0.008888888888888889_f64 * t13565 * t13566 * t12398 - 0.12_f64 * t13565 * t13570 * t12398 + 0.04_f64 * t13565 * t13574 * t12398 - 1.1757277777777777_f64 * t12329 - 0.14396666666666666_f64 * t12332 + 0.4319_f64 * t12335 + 0.03732469135802469_f64 * t12337 - 0.8638_f64 * t12341 - 1.2957_f64 * t12345 + 0.47988888888888886_f64 * t12348 + 0.8638_f64 * t12351 + 0.5278777777777778_f64 * t12354 - 0.07198333333333333_f64 * t12356 - 1.5836333333333332_f64 * t12358 + 0.023994444444444443_f64 * t12360 + 0.03999074074074074_f64 * t12362;
    t13591
}
