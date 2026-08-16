//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 994/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk994(t15274: f64, t1746: f64, t5949: f64, t5686: f64, t5688: f64, t5697: f64, t5950: f64, t5702: f64, t3010: f64, t3156: f64, t3161: f64, t3173: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15275 = 0.9598512193592288_f64 * t15274;
    let t15296 = t5949 * t1746;
    let t15297 = 2.0538164420033334_f64 * t15296;
    let t15306 = 6.0_f64 * t5686;
    let t15307 = 24.0_f64 * t5688;
    let t15311 = 24.0_f64 * t5697;
    let t15312 = 2.464579730404_f64 * t5950;
    let t15315 = 0.0010986933022051897_f64 * t5702;
    let t15316 = 24.0_f64 * t3010;
    let t15321 = 48.0_f64 * t3156;
    let t15322 = 480.0_f64 * t3161;
    let t15323 = 192.0_f64 * t3173;
    (t15275, t15297, t15306, t15307, t15311, t15312, t15315, t15316, t15321, t15322, t15323)
}
