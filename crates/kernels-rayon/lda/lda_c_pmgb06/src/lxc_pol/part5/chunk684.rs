//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 684/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk684(t2642: f64, t405: f64, t2645: f64, t2639: f64, t443: f64, t5961: f64, t473: f64, t453: f64, t36: f64, t103: f64, t2060: f64, t6177: f64, t6180: f64, t6183: f64, t6187: f64, t6191: f64, t6193: f64, t6196: f64, t6199: f64, t6202: f64, t6205: f64, t6207: f64, t6209: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6211 = t405 * t2642;
    let t6213 = t405 * t2645;
    let t6215 = t405 * t2639;
    let t6217 = t443 * t5961;
    let t6218 = t473 * t6217;
    let t6221 = t453 * t6217;
    let t6222 = t36 * t6221;
    let t6224 = 0.14396666666666666_f64 * t6177 - 0.03999074074074074_f64 * t6180 - 0.09597777777777777_f64 * t6183 - 0.21595_f64 * t6187 + 0.2879333333333333_f64 * t6191 - 0.04_f64 * t103 * t6193 + 0.05333333333333334_f64 * t2060 * t6196 + 0.013333333333333334_f64 * t103 * t6199 - 0.0022222222222222222_f64 * t103 * t6202 + 0.007998148148148148_f64 * t6205 - 0.023994444444444443_f64 * t6207 + 0.011997222222222222_f64 * t6209 - 0.008888888888888889_f64 * t6211 + 0.0044444444444444444_f64 * t6213 + 0.0014814814814814814_f64 * t6215 - 0.006666666666666667_f64 * t103 * t6218 - 0.035991666666666665_f64 * t6222;
    (t6211, t6213, t6215, t6217, t6218, t6221, t6222, t6224)
}
