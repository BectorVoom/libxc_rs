//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 879/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk879(t1005: f64, t1010: f64, t1013: f64, t155: f64, t174: f64, t2946: f64, t2983: f64, t2986: f64, t3027: f64, t3064: f64, t3076: f64, t3082: f64, t3086: f64, t3095: f64, t3098: f64, t371: f64, t380: f64, t386: f64, t473: f64, t75: f64, t8164: f64, t8170: f64, t8171: f64, t8174: f64, t8389: f64, t8393: f64, t8397: f64, t8400: f64, t8414: f64, t8417: f64, t8441: f64, t8564: f64, t8586: f64, t989: f64, t991: f64) -> f64 {
    let t8639 = -0.08674864706225219_f64 * t174 * t473 * t1010 * t1013 + t8389 + t8393 - t8397 + t8400 - 38.02486811957057_f64 * t174 * t155 * t2983 * t3082 - 24.0_f64 * t3076 * t8564 * t371 + 0.5848223397455204_f64 * t380 * t8164 * t386 + 91080.98259910992_f64 * t75 * t8170 * t8171 * t8174 + 1157.9285944033452_f64 * t3064 * t8564 * t991 - t8414 - t8417 + 96.4940495336121_f64 * t989 * t8586 * t991 - 12304.676425209354_f64 * t75 * t8441 * t8171 * t2986 + 3.8527556876111295_f64 * t174 * t155 * t2946 * t3086 - 0.021687161765563047_f64 * t174 * t1005 * t3095 - 0.1301229705933783_f64 * t174 * t3027 * t3098;
    t8639
}
