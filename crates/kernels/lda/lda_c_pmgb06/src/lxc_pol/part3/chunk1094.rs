//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1094/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1094<F: Float>(t154: F, t3092: F, t12398: F, t13026: F, t465: F, t13002: F, t1395: F, t1438: F, t5083: F, t5086: F, t12987: F, t5084: F) -> (F, F, F, F) {
    let t13027 = t154 * t3092;
    let t13030 = F::new(8.0) / F::new(27.0) * t13026 * t13027 * t12398;
    let t13031 = t465 * t3092;
    let t13034 = F::new(8.0) / F::new(27.0) * t13026 * t13031 * t13002;
    let t13035 = t1395 * t1438;
    let t13038 = F::new(2.0) / F::new(9.0) * t5083 * t13035 * t5086;
    let t13041 = t5083 * t5084 * t12987 / F::new(9.0);
    (t13030, t13034, t13038, t13041)
}
