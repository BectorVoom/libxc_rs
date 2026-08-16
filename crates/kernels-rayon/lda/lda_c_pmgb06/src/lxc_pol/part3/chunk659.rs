//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 659/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk659(t1147: f64, t123: f64, t317: f64, t701: f64, t1126: f64, t740: f64, t1312: f64, t1316: f64, t1317: f64, t1323: f64, t2180: f64, t346: f64, t388: f64, t3995: f64, t3999: f64, t4005: f64, t4006: f64, t4013: f64, t4017: f64, t4021: f64, t4027: f64) -> (f64, f64, f64) {
    let t4030 = t123 * t1147 * t701 * t317;
    let t4034 = t123 * t740 * t1126 * t317;
    let t4036 = 0.004067943812504169_f64 * t3995 + t3999 - t4005 + 9.0_f64 * t1316 * t388 * t4006 + 9.0_f64 * t1316 * t1312 * t1317 - 2.0_f64 * t346 * t1323 * t4013 + 9.0_f64 * t1316 * t388 * t4017 + 18.0_f64 * t2180 * t388 * t4021 - t4027 + 0.5945049527603057_f64 * t4030 - 0.16213771438917426_f64 * t4034;
    (t4030, t4034, t4036)
}
