//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 741/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk741(t2057: f64, t405: f64, t2054: f64, t103: f64, t2060: f64, t3396: f64, t3398: f64, t3400: f64, t3413: f64, t3414: f64, t4635: f64, t4642: f64, t5006: f64, t5010: f64, t5013: f64, t5016: f64, t5019: f64, t5022: f64, t5025: f64, t5028: f64) -> f64 {
    let t5032 = 0.017777777777777778_f64 * t405 * t2057;
    let t5034 = 0.002962962962962963_f64 * t405 * t2054;
    let t5038 = 0.057777777777777775_f64 * t5006 - 0.015996296296296297_f64 * t4635 + 0.2639388888888889_f64 * t4642 - t3413 - t3414 + 0.013333333333333334_f64 * t103 * t5010 - 0.04_f64 * t103 * t5013 - 0.0022222222222222222_f64 * t103 * t5016 - 0.002962962962962963_f64 * t103 * t5019 - 0.008888888888888889_f64 * t2060 * t5022 + 0.013333333333333334_f64 * t103 * t5025 + 0.05333333333333334_f64 * t2060 * t5028 - t5032 + t5034 - 0.014814814814814815_f64 * t3396 + 0.0044444444444444444_f64 * t3398 + 0.0014814814814814814_f64 * t3400;
    t5038
}
