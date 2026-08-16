//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 880/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk880(t1023: f64, t1027: f64, t1030: f64, t1031: f64, t1124: f64, t155: f64, t174: f64, t3020: f64, t3031: f64, t3038: f64, t3059: f64, t3063: f64, t3067: f64, t3068: f64, t3071: f64, t3075: f64, t3077: f64, t3081: f64, t364: f64, t372: f64, t379: f64, t387: f64, t473: f64, t62: f64, t8171: f64, t8423: f64, t8427: f64, t8473: f64, t8477: f64, t8505: f64, t8564: f64, t966: f64, t970: f64, t984: f64, t987: f64, t988: f64, t992: f64) -> f64 {
    let t8686 = -0.06747116993730726_f64 * t174 * t1124 * t379 * t387 + 0.13698666666666667_f64 * t174 * t3031 * t984 + 4.406132732925914_f64 * t174 * t473 * t988 * t992 - t8423 + 1.2842518958703766_f64 * t174 * t473 * t1027 * t1031 + t8427 - 0.21309037037037037_f64 * t174 * t1124 * t364 * t372 + 0.043374323531126094_f64 * t174 * t3020 * t1023 - 0.06849333333333334_f64 * t174 * t966 * t3059 - 141.7218633942076_f64 * t174 * t155 * t3063 * t3068 - 0.41096_f64 * t174 * t3038 * t3071 + 13.218398198777741_f64 * t174 * t155 * t3075 * t3077 + 623.3672123775311_f64 * t3081 * t8171 * t1030 - 24829.60425438716_f64 * t62 / t987 / t970 * t8564 * t3067 - t8473 + t8477 + t8505;
    t8686
}
