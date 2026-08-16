//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1212/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1212(t1319: f64, t21777: f64, t571: f64, t2017: f64, t21794: f64, t21219: f64, t4758: f64, t1446: f64, t7684: f64, t17901: f64, t17906: f64, t15579: f64, t2007: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t21897 = 32.0_f64 / 15.0_f64 * t571 * t1319 * t21777;
    let t21900 = 16.0_f64 / 3.0_f64 * t571 * t2017 * t21794;
    let t21903 = 16.0_f64 / 5.0_f64 * t571 * t4758 * t21219;
    let t21905 = 8.0_f64 / 15.0_f64 * t1446 * t7684;
    let t21906 = 32.0_f64 / 45.0_f64 * t17901;
    let t21907 = 64.0_f64 / 45.0_f64 * t17906;
    let t21909 = 8.0_f64 / 15.0_f64 * t15579 * t2007;
    (t21897, t21900, t21903, t21905, t21906, t21907, t21909)
}
