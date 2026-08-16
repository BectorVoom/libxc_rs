//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 916/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk916<F: Float>(t5: F, t131: F, t6687: F, t178: F, t1887: F, t815: F, t1874: F, t802: F, t1: F, t760: F, t2381: F, t332: F, t395: F, t5961: F, zeta_threshold: F) -> (F, F, F, F, F, F) {
    let t6 = t5 <= zeta_threshold;
    let t6688 = t6687 * t131;
    let t6690 = t6688 * t178 / F::cast_from(30.0_f64);
    let t6692 = t1887 * t815 / F::cast_from(15.0_f64);
    let t6694 = t802 * t1874 / F::cast_from(15.0_f64);
    let t6695 = t760 * t1;
    let t6698 = t332 * t2381;
    let t6703 = piecewise3::<F>(t6, F::cast_from(0.0_f64), F::cast_from(8.0_f64) * t6695 * t395 + F::cast_from(2.0_f64) * t5 * t5961 + F::cast_from(2.0_f64) * t6698);
    (t6688, t6690, t6692, t6694, t6695, t6703)
}
