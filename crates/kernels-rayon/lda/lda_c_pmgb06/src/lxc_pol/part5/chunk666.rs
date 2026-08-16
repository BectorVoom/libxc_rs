//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 666/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk666(t342: f64, t787: f64, t374: f64, t2695: f64, t73: f64, t388: f64, t1316: f64, t1324: f64, t2180: f64, t2255: f64, t2308: f64, t2733: f64, t346: f64, t384: f64, t3987: f64, t3991: f64, t3995: f64, t3999: f64, t4005: f64, t4355: f64, t4360: f64, t5583: f64, t5999: f64, t6006: f64, t6009: f64, t6013: f64, t6018: f64, t6021: f64, t790: f64) -> (f64, f64, f64, f64) {
    let t6024 = t787 * t342;
    let t6028 = t787 * t374;
    let t6031 = t73 * t2695;
    let t6032 = t388 * t6031;
    let t6035 = t346 * t790 * t2255 + 12.0_f64 * t2180 * t5999 + t346 * t2733 * t384 - 0.0005811348303577384_f64 * t3987 - t3991 + 0.001355981270834723_f64 * t3995 + t3999 - t4005 + 2.0_f64 * t6006 * t6009 - 3.0_f64 * t5583 * t6013 - 6.0_f64 * t5583 * t4355 + 12.0_f64 * t6018 * t4360 - t346 * t6021 * t1324 + 3.0_f64 * t1316 * t790 * t6024 - t346 * t2308 * t6028 + 6.0_f64 * t2180 * t6032;
    (t6024, t6028, t6031, t6035)
}
