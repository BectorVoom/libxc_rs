//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1167/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1167<F: Float>(t10439: F, t21022: F, t332: F, t439: F, t2002: F, t6413: F, t132: F, t435: F, t7862: F, t1897: F, t19782: F, t2010: F) -> (F, F, F, F) {
    let t21026 = F::new(2.0) / F::new(15.0) * t439 * t10439 * t21022 * t332;
    let t21028 = t2002 * t6413 / F::new(15.0);
    let t21032 = t132 * t435 * t7862;
    let t21033 = t21032 / F::new(15.0);
    let t21036 = F::new(4.0) / F::new(15.0) * t2010 * t1897 * t19782;
    (t21026, t21028, t21033, t21036)
}
