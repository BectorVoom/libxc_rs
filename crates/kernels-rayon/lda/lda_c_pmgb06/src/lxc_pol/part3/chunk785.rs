//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 785/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk785(t2021: f64, t97: f64, t588: f64, t1499: f64, t844: f64, t1837: f64, t486: f64, t4876: f64, t2900: f64, t2901: f64, t2903: f64, t2905: f64, t2907: f64, t4859: f64, t4863: f64, t4868: f64, t4871: f64, t4874: f64, t4878: f64, t4882: f64, t4887: f64, t4911: f64, t4916: f64, t4924: f64) -> (f64, f64, f64, f64, f64) {
    let t5391 = t2021 * t97;
    let t5393 = 0.12155555555555556_f64 * t5391 * t588;
    let t5396 = t1499 * t844 / 30.0_f64;
    let t5398 = t486 * t1837 / 15.0_f64;
    let t5405 = 0.002518888888888889_f64 * t4876;
    let t5415 = t2900 + 0.0016792592592592592_f64 * t2901 - 0.0004198148148148148_f64 * t2903 + 0.0012594444444444445_f64 * t2905 - 0.0006297222222222223_f64 * t2907 + 0.0008396296296296296_f64 * t4911 - 0.0008396296296296296_f64 * t4878 + t5405 + 0.01385388888888889_f64 * t4916 + 0.002099074074074074_f64 * t4887 - 0.007556666666666666_f64 * t4859 - 0.005037777777777778_f64 * t4868 + 0.0012594444444444445_f64 * t4882 + 0.011335_f64 * t4863 + 0.015113333333333333_f64 * t4874 - 0.003778333333333333_f64 * t4871 - 0.003778333333333333_f64 * t4924;
    (t5391, t5393, t5396, t5398, t5415)
}
