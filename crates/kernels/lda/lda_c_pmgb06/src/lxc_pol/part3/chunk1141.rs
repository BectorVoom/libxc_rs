//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1141/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1141<F: Float>(t12329: F, t12332: F, t12335: F, t12337: F, t12341: F, t12345: F, t12348: F, t12351: F, t12354: F, t12356: F, t12358: F, t12360: F, t12362: F, t12398: F, t13558: F, t13561: F, t13565: F, t13566: F, t13570: F, t13574: F, t2060: F, t2969: F, t473: F, t9225: F) -> F {
    let t13591 = F::new(0.08) * t2060 * t473 * t2969 + F::new(0.019753086419753086) * t13558 - F::new(0.28444444444444444) * t13561 + F::new(0.023994444444444443) * t9225 - F::new(0.008888888888888889) * t13565 * t13566 * t12398 - F::new(0.12) * t13565 * t13570 * t12398 + F::new(0.04) * t13565 * t13574 * t12398 - F::new(1.1757277777777777) * t12329 - F::new(0.14396666666666666) * t12332 + F::new(0.4319) * t12335 + F::new(0.03732469135802469) * t12337 - F::new(0.8638) * t12341 - F::new(1.2957) * t12345 + F::new(0.47988888888888886) * t12348 + F::new(0.8638) * t12351 + F::new(0.5278777777777778) * t12354 - F::new(0.07198333333333333) * t12356 - F::new(1.5836333333333332) * t12358 + F::new(0.023994444444444443) * t12360 + F::new(0.03999074074074074) * t12362;
    t13591
}
