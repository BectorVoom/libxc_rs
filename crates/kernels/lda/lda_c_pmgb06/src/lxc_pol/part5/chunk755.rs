//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 755/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk755<F: Float>(t3311: F, t3324: F, t3327: F, t3331: F, t3335: F, t5675: F, t7750: F, t7752: F, t7754: F, t7756: F, t7758: F, t7759: F, t7760: F, t7761: F, t7762: F, t7763: F) -> (F,) {
    let t7764 = -t7750 - t7752 - t7754 - t7756 + 8.0 * t5675 - t3311 + t3324 + t3327 + t3331 - t3335 - t7758 - t7759 - t7760 - t7761 - t7762 - t7763;
    (t7764,)
}
