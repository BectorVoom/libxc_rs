//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1025/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1025(t2146: f64, t3873: f64, t3841: f64, t1446: f64, t4834: f64, t5234: f64, t5238: f64, t4804: f64, t5276: f64, t3794: f64, t3476: f64, t5146: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12012 = 8.0_f64 / 15.0_f64 * t2146 * t3873;
    let t12014 = 8.0_f64 / 15.0_f64 * t2146 * t3841;
    let t12015 = t1446 * t4834;
    let t12016 = 16.0_f64 / 45.0_f64 * t12015;
    let t12017 = t1446 * t5234;
    let t12018 = 32.0_f64 / 45.0_f64 * t12017;
    let t12019 = t1446 * t5238;
    let t12020 = 16.0_f64 / 27.0_f64 * t12019;
    let t12022 = 8.0_f64 / 15.0_f64 * t4804 * t5276;
    let t12024 = 8.0_f64 / 15.0_f64 * t3794 * t5276;
    let t12025 = t5146 * t3476;
    (t12012, t12014, t12016, t12018, t12020, t12022, t12024, t12025)
}
