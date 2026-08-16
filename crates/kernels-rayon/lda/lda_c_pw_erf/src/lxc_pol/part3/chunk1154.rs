//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1154/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1154(t1472: f64, t5342: f64, t1184: f64, t2152: f64, t571: f64, t573: f64, t1446: f64, t5339: f64, t13486: f64, t13489: f64, t13491: f64, t13494: f64, t13496: f64, t13498: f64, t13500: f64, t13505: f64, t13508: f64, t13510: f64) -> (f64, f64, f64, f64) {
    let t13511 = t1472 * t5342;
    let t13512 = 8.0_f64 / 135.0_f64 * t13511;
    let t13515 = t571 * t1184 * t573 * t2152;
    let t13516 = 128.0_f64 / 135.0_f64 * t13515;
    let t13517 = t1446 * t5339;
    let t13518 = 8.0_f64 / 135.0_f64 * t13517;
    let t13519 = -t13486 + t13489 - t13491 - t13494 - t13496 + t13498 + t13500 + t13505 + t13508 + t13510 - t13512 - t13516 - t13518;
    (t13512, t13516, t13518, t13519)
}
