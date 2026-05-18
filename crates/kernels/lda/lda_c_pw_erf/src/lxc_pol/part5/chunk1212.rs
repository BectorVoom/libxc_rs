//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1212/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1212<F: Float>(t1319: F, t21777: F, t571: F, t2017: F, t21794: F, t21219: F, t4758: F, t1446: F, t7684: F, t17901: F, t17906: F, t15579: F, t2007: F) -> (F, F, F, F, F, F, F) {
    let t21897 = F::new(32.0) / F::new(15.0) * t571 * t1319 * t21777;
    let t21900 = F::new(16.0) / F::new(3.0) * t571 * t2017 * t21794;
    let t21903 = F::new(16.0) / F::new(5.0) * t571 * t4758 * t21219;
    let t21905 = F::new(8.0) / F::new(15.0) * t1446 * t7684;
    let t21906 = F::new(32.0) / F::new(45.0) * t17901;
    let t21907 = F::new(64.0) / F::new(45.0) * t17906;
    let t21909 = F::new(8.0) / F::new(15.0) * t15579 * t2007;
    (t21897, t21900, t21903, t21905, t21906, t21907, t21909)
}
