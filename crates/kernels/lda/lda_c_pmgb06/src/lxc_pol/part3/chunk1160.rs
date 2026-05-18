//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1160/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1160<F: Float>(t13848: F, t5474: F, t5499: F, t1380: F, t337: F, t493: F, t4935: F, t497: F, t13834: F, t13835: F, t13837: F, t13839: F, t13841: F, t13843: F, t13845: F, t13847: F) -> (F, F, F, F) {
    let t13849 = F::new(16.0) / F::new(81.0) * t13848;
    let t13850 = t5499 * t5474;
    let t13851 = F::new(10.0) / F::new(27.0) * t13850;
    let t13856 = t493 * t1380 * t4935 * t497 * t337 / F::new(15.0);
    let t13857 = t13834 - t13835 - t13837 - t13839 - t13841 + t13843 + t13845 + t13847 + t13849 + t13851 - t13856;
    (t13849, t13851, t13856, t13857)
}
