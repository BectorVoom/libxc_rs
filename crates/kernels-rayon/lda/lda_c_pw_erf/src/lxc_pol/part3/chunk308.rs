//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 308/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk308(t1012: f64, t1030: f64, t1001: f64, t1005: f64, t1011: f64, t1013: f64, t1023: f64, t1028: f64, t174: f64, t365: f64, t372: f64, t380: f64, t387: f64, t4: f64, t474: f64, t71: f64, t84: f64, t910: f64, t916: f64, t938: f64, t966: f64, t972: f64, t974: f64, t984: f64, t989: f64, t992: f64, t997: f64) -> (f64, f64) {
    let t1031 = t1012 * t1030;
    let t1034 = -0.0007098192444444445_f64 * t4 * t474 * t71 - 0.03424666666666667_f64 * t174 * t966 * t372 - 2.0_f64 * t972 * t974 + 1.0_f64 * t365 * t984 + 32.1646831778707_f64 * t989 * t992 + t997 + t1001 + t916 - t938 - t910 - 0.0002441540671567088_f64 * t4 * t474 * t84 - 0.010843580882781523_f64 * t174 * t1005 * t387 - 1.169644679491041_f64 * t1011 * t1013 + 0.5848223397455204_f64 * t380 * t1023 + 17.315755899375862_f64 * t1028 * t1031;
    (t1031, t1034)
}
