//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 801/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk801(t1996: f64, t3802: f64, t519: f64, t1251: f64, t806: f64, t940: f64, t1313: f64, t1318: f64, t1325: f64, t1446: f64, t1454: f64, t1462: f64, t2171: f64, t2178: f64, t221: f64, t3709: f64, t5373: f64, t5375: f64, t5380: f64, t5382: f64, t5394: f64, t5399: f64, t5401: f64, t5406: f64, t5411: f64, t5414: f64, t5418: f64, t571: f64, t799: f64) -> (f64, f64, f64, f64) {
    let t5421 = t3802 * t1996;
    let t5423 = 16.0_f64 / 135.0_f64 * t519 * t5421;
    let t5424 = t806 * t1251;
    let t5425 = t5424 * t940;
    let t5426 = t1313 * t5425;
    let t5429 = t5373 + 4.0_f64 / 15.0_f64 * t571 * t5375 - t5380 - 4.0_f64 / 15.0_f64 * t1325 * t5382 + 4.0_f64 / 45.0_f64 * t2171 * t1454 + 4.0_f64 / 27.0_f64 * t2171 * t1462 + 4.0_f64 / 45.0_f64 * t3709 * t799 + 16.0_f64 / 45.0_f64 * t1446 * t2178 - 4.0_f64 / 15.0_f64 * t1325 * t5394 + t5399 + 4.0_f64 / 15.0_f64 * t5401 * t221 - 16.0_f64 / 45.0_f64 * t1318 * t5406 + t5411 - 16.0_f64 / 45.0_f64 * t1325 * t5414 + 16.0_f64 / 45.0_f64 * t1325 * t5418 - t5423 + 8.0_f64 / 45.0_f64 * t519 * t5426;
    (t5421, t5425, t5426, t5429)
}
