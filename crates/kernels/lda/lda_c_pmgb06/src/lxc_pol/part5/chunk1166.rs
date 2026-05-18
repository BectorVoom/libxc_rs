//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1166/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1166<F: Float>(t12592: F, t19377: F, t493: F, t19381: F, t1981: F, t5470: F, t497: F, t7806: F, t337: F, t9908: F, t443: F, t7811: F) -> (F, F, F, F) {
    let t21013 = F::new(88.0) / F::new(243.0) * t493 * t12592 * t19377;
    let t21016 = F::new(16.0) / F::new(27.0) * t1981 * t5470 * t19381;
    let t21017 = t7806 * t497;
    let t21021 = F::new(2.0) / F::new(15.0) * t493 * t9908 * t21017 * t337;
    let t21022 = t7811 * t443;
    (t21013, t21016, t21021, t21022)
}
