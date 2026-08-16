//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 972/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk972(t3138: f64, t3150: f64, t3167: f64, t3175: f64, t4394: f64, t2705: f64, t2740: f64, t2743: f64, t4385: f64, t7324: f64, t7325: f64, t7326: f64, t7327: f64, t7328: f64, t7329: f64, t7330: f64, t7332: f64, t8097: f64, t8098: f64, t8099: f64, t8101: f64, t8102: f64, t8103: f64) -> (f64, f64, f64, f64, f64) {
    let t11276 = 385.9637837316265_f64 * t3138;
    let t11277 = 4.0_f64 * t3150;
    let t11282 = 480.0_f64 * t3167;
    let t11286 = 240.0_f64 * t3175;
    let t11299 = 3.5089340384731225_f64 * t4394;
    let t11302 = 3.0_f64 * t4385 - t8097 + t8098 + t8099 + 0.09759222794503372_f64 * t2705 - t8101 - t8102 + t7324 - t7325 - t7326 - t11299 - t8103 - 5.263401057709683_f64 * t2740 + t7327 - 3.5089340384731225_f64 * t2743 + t7328 - t7329 + t7330 + t7332;
    (t11276, t11277, t11282, t11286, t11302)
}
