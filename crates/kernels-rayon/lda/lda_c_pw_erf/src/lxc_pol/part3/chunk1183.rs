//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1183/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1183(t13938: f64, t10027: f64, t5157: f64, t13777: f64, t3974: f64, t3976: f64, t593: f64, t13914: f64, t13916: f64, t13917: f64, t13919: f64, t13923: f64, t13925: f64, t13927: f64, t13930: f64, t13933: f64, t13937: f64) -> (f64, f64, f64, f64) {
    let t13939 = 32.0_f64 / 27.0_f64 * t13938;
    let t13941 = 16.0_f64 / 15.0_f64 * t10027 * t5157;
    let t13945 = 8.0_f64 / 15.0_f64 * t3974 * t3976 * t13777 * t593;
    let t13946 = t13914 + t13916 + 0.21642082724729686_f64 * t13917 - 0.09618703433213194_f64 * t13919 - t13923 - t13925 - t13927 - t13930 + t13933 + t13937 + t13939 - t13941 - t13945;
    (t13939, t13941, t13945, t13946)
}
