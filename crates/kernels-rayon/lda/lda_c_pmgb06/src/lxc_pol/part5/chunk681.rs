//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 681/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk681(t1438: f64, t2381: f64, t332: f64, t1525: f64, t36: f64, t103: f64, t2060: f64, t3082: f64, t3396: f64, t3413: f64, t3414: f64, t4635: f64, t4639: f64, t4642: f64, t5002: f64, t5003: f64, t5006: f64, t5032: f64, t5034: f64, t6147: f64, t6152: f64, t6156: f64, t6162: f64) -> (f64, f64, f64, f64) {
    let t6164 = t1438 * t2381;
    let t6165 = t6164 * t332;
    let t6166 = t1525 * t6165;
    let t6167 = t36 * t6166;
    let t6175 = -0.015996296296296297_f64 * t3082 + 0.013333333333333334_f64 * t103 * t6147 - 0.002962962962962963_f64 * t103 * t6152 - 0.008888888888888889_f64 * t2060 * t6156 + 0.07198333333333333_f64 * t6162 - 0.023994444444444443_f64 * t6167 - 0.047988888888888886_f64 * t4639 + t5002 - 0.014814814814814815_f64 * t5003 + 0.017777777777777778_f64 * t5006 - 0.03199259259259259_f64 * t4635 + 0.047988888888888886_f64 * t4642 - t3413 - t3414 - t5032 + t5034 - 0.007407407407407408_f64 * t3396;
    (t6165, t6166, t6167, t6175)
}
