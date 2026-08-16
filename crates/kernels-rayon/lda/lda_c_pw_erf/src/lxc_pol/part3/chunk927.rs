//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 927/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk927(t10030: f64, t3979: f64, t3994: f64, t515: f64, t174: f64, t205: f64, t9810: f64, t325: f64, t3648: f64, t1332: f64, t1350: f64, t3640: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10031 = t10030 * t3979;
    let t10039 = t3994 * t515;
    let t10042 = t174 * t9810 * t205;
    let t10043 = 0.01959135802469136_f64 * t10042;
    let t10053 = t325 * t3648;
    let t10056 = 1.0_f64 / t1350 / t1332;
    let t10066 = t325 * t3640;
    (t10031, t10039, t10042, t10043, t10053, t10056, t10066)
}
