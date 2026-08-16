//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 881/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk881(t2869: f64, t31: f64, t4: f64, t1011: f64, t1012: f64, t1013: f64, t1022: f64, t1028: f64, t1030: f64, t2735: f64, t3058: f64, t3059: f64, t3064: f64, t3067: f64, t3076: f64, t3081: f64, t3085: f64, t3101: f64, t3111: f64, t370: f64, t385: f64, t387: f64, t71: f64, t8370: f64, t84: f64, t8509: f64, t8533: f64, t8536: f64, t8539: f64, t8542: f64, t972: f64, t973: f64, t974: f64, t983: f64, t989: f64, t991: f64) -> (f64, f64) {
    let t8716 = 0.011483710345679013_f64 * t4 * t2869 * t31;
    let t8729 = -t8509 + 36.0_f64 * t989 * t974 * t983 - t8533 + t8536 - t8539 + t8542 + 128.6587327114828_f64 * t989 * t3058 * t991 * t370 - 1157.9285944033452_f64 * t3076 * t3101 * t973 + 12414.80212719358_f64 * t3064 * t983 * t3067 * t973 - 4.678578717964164_f64 * t1011 * t387 * t2735 + 6152.338212604677_f64 * t3081 * t8370 * t1012 + 21.053604230838733_f64 * t1028 * t1013 * t1022 + 69.26302359750345_f64 * t1028 * t2735 * t1030 * t385 + t8716 - 0.005520816345679013_f64 * t4 * t2869 * t71 - 8.0_f64 * t972 * t3059 * t370 - 623.3672123775311_f64 * t3085 * t3111 * t1012 - 0.0018989760778855128_f64 * t4 * t2869 * t84;
    (t8716, t8729)
}
