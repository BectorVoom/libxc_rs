//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 812/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk812<F: Float>(t2869: F, t31: F, t4: F, t1011: F, t1012: F, t1013: F, t1022: F, t1028: F, t1030: F, t2735: F, t3058: F, t3059: F, t3064: F, t3067: F, t3076: F, t3081: F, t3085: F, t3101: F, t3111: F, t370: F, t385: F, t387: F, t71: F, t8370: F, t84: F, t8509: F, t8533: F, t8536: F, t8539: F, t8542: F, t972: F, t973: F, t974: F, t983: F, t989: F, t991: F) -> (F, F) {
    let t8716 = 0.011483710345679013 * t4 * t2869 * t31;
    let t8729 = -t8509 + 36.0 * t989 * t974 * t983 - t8533 + t8536 - t8539 + t8542 + 128.6587327114828 * t989 * t3058 * t991 * t370 - 1157.9285944033452 * t3076 * t3101 * t973 + 12414.80212719358 * t3064 * t983 * t3067 * t973 - 4.678578717964164 * t1011 * t387 * t2735 + 6152.338212604677 * t3081 * t8370 * t1012 + 21.053604230838733 * t1028 * t1013 * t1022 + 69.26302359750345 * t1028 * t2735 * t1030 * t385 + t8716 - 0.005520816345679013 * t4 * t2869 * t71 - 8.0 * t972 * t3059 * t370 - 623.3672123775311 * t3085 * t3111 * t1012 - 0.0018989760778855128 * t4 * t2869 * t84;
    (t8716, t8729)
}
