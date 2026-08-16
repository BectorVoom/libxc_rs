//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 929/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk929(t525: f64, t6406: f64, t6760: f64, t1576: f64, t6765: f64, t103: f64, t2060: f64, t3368: f64, t3380: f64, t6800: f64, t6811: f64, t6819: f64, t6829: f64, t6873: f64, t6875: f64, t6877: f64, t6879: f64, t6882: f64, t6885: f64, t6888: f64, t6891: f64) -> (f64, f64, f64, f64) {
    let t6894 = t525 * t6406;
    let t6897 = t525 * t6760;
    let t6900 = t1576 * t6765;
    let t6903 = -t3368 - t3380 + 0.007998148148148148_f64 * t6800 - 0.023994444444444443_f64 * t6811 + 0.011997222222222222_f64 * t6819 - 0.035991666666666665_f64 * t6829 - 0.008888888888888889_f64 * t6873 + 0.0044444444444444444_f64 * t6875 + 0.0014814814814814814_f64 * t6877 - 0.006666666666666667_f64 * t103 * t6879 + 0.013333333333333334_f64 * t103 * t6882 - 0.002962962962962963_f64 * t103 * t6885 + 0.008888888888888889_f64 * t2060 * t6888 - 0.04_f64 * t103 * t6891 - 0.05333333333333334_f64 * t2060 * t6894 + 0.013333333333333334_f64 * t103 * t6897 - 0.0022222222222222222_f64 * t103 * t6900;
    (t6894, t6897, t6900, t6903)
}
