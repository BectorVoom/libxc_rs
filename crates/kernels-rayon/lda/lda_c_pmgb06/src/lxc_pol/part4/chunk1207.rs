//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1207/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1207(t493: f64, t5276: f64, t5486: f64, t12617: f64, t5281: f64, t1083: f64, t1380: f64, t6781: f64, t15879: f64, t15883: f64, t15886: f64, t15888: f64, t15890: f64, t15892: f64, t15894: f64, t15896: f64, t15898: f64, t15900: f64, t15902: f64, t15905: f64) -> (f64, f64, f64, f64) {
    let t15908 = 2.0_f64 / 45.0_f64 * t493 * t5486 * t5276;
    let t15911 = 2.0_f64 / 27.0_f64 * t493 * t12617 * t5281;
    let t15915 = t493 * t1380 * t6781 * t1083 / 45.0_f64;
    let t15916 = t15879 + t15883 + t15886 - t15888 - t15890 - t15892 - t15894 + t15896 - t15898 - t15900 - t15902 - t15905 - t15908 - t15911 - t15915;
    (t15908, t15911, t15915, t15916)
}
