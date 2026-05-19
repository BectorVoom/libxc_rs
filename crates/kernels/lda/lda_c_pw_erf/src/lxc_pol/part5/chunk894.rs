//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 894/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk894<F: Float>(t1005: F, t1010: F, t1013: F, t155: F, t174: F, t2946: F, t2983: F, t2986: F, t3027: F, t3064: F, t3076: F, t3082: F, t3086: F, t3095: F, t3098: F, t371: F, t380: F, t386: F, t473: F, t75: F, t8164: F, t8170: F, t8171: F, t8174: F, t8389: F, t8393: F, t8397: F, t8400: F, t8414: F, t8417: F, t8441: F, t8564: F, t8586: F, t989: F, t991: F) -> F {
    let t8639 = -F::cast_from(0.08674864706225219_f64) * t174 * t473 * t1010 * t1013 + t8389 + t8393 - t8397 + t8400 - F::cast_from(38.02486811957057_f64) * t174 * t155 * t2983 * t3082 - F::new(24.0) * t3076 * t8564 * t371 + F::cast_from(0.5848223397455204_f64) * t380 * t8164 * t386 + F::cast_from(91080.98259910992_f64) * t75 * t8170 * t8171 * t8174 + F::cast_from(1157.9285944033452_f64) * t3064 * t8564 * t991 - t8414 - t8417 + F::cast_from(96.4940495336121_f64) * t989 * t8586 * t991 - F::cast_from(12304.676425209354_f64) * t75 * t8441 * t8171 * t2986 + F::cast_from(3.8527556876111295_f64) * t174 * t155 * t2946 * t3086 - F::cast_from(0.021687161765563047_f64) * t174 * t1005 * t3095 - F::cast_from(0.1301229705933783_f64) * t174 * t3027 * t3098;
    t8639
}
