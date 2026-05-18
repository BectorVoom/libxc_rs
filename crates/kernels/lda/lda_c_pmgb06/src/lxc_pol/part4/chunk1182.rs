//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1182/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1182<F: Float>(t147: F, t1669: F, t99: F, t12329: F, t12337: F, t12354: F, t12356: F, t12358: F, t13558: F, t13561: F, t13570: F, t13574: F, t15324: F, t15326: F, t15329: F, t15332: F) -> (F, F) {
    let t15548 = t99 * t1669 * t147;
    let t15563 = F::new(1.1517333333333333) * t15326 - F::new(0.31992592592592595) * t15329 + F::new(0.10666666666666667) * t15548 * t13574 * t15324 - F::new(1.7276) * t15332 - F::new(0.32) * t15548 * t13570 * t15324 + F::new(0.03950617283950617) * t13558 - F::new(0.2725925925925926) * t13561 - F::new(1.135737037037037) * t12329 + F::new(0.07464938271604939) * t12337 + F::new(0.06398518518518519) * t12354 - F::new(0.047988888888888886) * t12356 - F::new(0.19195555555555555) * t12358;
    (t15548, t15563)
}
