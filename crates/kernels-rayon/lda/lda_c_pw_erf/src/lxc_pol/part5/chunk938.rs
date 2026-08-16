//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 938/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk938(t3117: f64, t3120: f64, t3124: f64, t3132: f64, t3138: f64, t3150: f64, t3167: f64, t3175: f64, t4394: f64, t2693: f64, t2695: f64, t887: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11272 = 192.98189186581325_f64 * t3117;
    let t11273 = 24.0_f64 * t3120;
    let t11274 = 24.0_f64 * t3124;
    let t11275 = 2069.0005882282467_f64 * t3132;
    let t11276 = 385.9637837316265_f64 * t3138;
    let t11277 = 4.0_f64 * t3150;
    let t11282 = 480.0_f64 * t3167;
    let t11286 = 240.0_f64 * t3175;
    let t11299 = 3.5089340384731225_f64 * t4394;
    let t11305 = t887 * t2693 * t2695;
    (t11272, t11273, t11274, t11275, t11276, t11277, t11282, t11286, t11299, t11305)
}
