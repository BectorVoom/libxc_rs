//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1002/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1002<F: Float>(t432: F, t5051: F, t132: F, t1547: F, t1873: F, t5385: F, t588: F, t97: F, t208: F, t213: F, t4463: F, t579: F) -> (F, F, F, F) {
    let t11914 = t432 * t5051;
    let t11915 = t11914 / F::new(45.0);
    let t11917 = t132 * t1547 * t1873;
    let t11918 = t11917 / F::new(45.0);
    let t11920 = t5385 * t97 * t588;
    let t11921 = F::cast_from(0.36466666666666664_f64) * t11920;
    let t11928 = t4463 * t579 * t208 * t213;
    (t11915, t11918, t11921, t11928)
}
