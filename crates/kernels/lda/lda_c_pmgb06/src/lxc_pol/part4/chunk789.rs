//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 789/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk789<F: Float>(t439: F, t5254: F, t1901: F, t4659: F, t153: F, t3260: F) -> (F, F, F, F) {
    let t5256 = F::new(2.0) / F::new(27.0) * t439 * t5254;
    let t5257 = t1901 * t4659;
    let t5259 = t439 * t5257 / F::new(27.0);
    let t5260 = t3260 * t153;
    (t5256, t5257, t5259, t5260)
}
