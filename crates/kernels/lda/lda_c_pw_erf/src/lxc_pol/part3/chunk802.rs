//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 802/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk802<F: Float>(t1023: F, t1027: F, t1030: F, t1031: F, t1124: F, t155: F, t174: F, t3020: F, t3031: F, t3038: F, t3059: F, t3063: F, t3067: F, t3068: F, t3071: F, t3075: F, t3077: F, t3081: F, t364: F, t372: F, t379: F, t387: F, t473: F, t62: F, t8171: F, t8423: F, t8427: F, t8473: F, t8477: F, t8505: F, t8564: F, t966: F, t970: F, t984: F, t987: F, t988: F, t992: F) -> (F,) {
    let t8686 = -0.06747116993730726 * t174 * t1124 * t379 * t387 + 0.13698666666666667 * t174 * t3031 * t984 + 4.406132732925914 * t174 * t473 * t988 * t992 - t8423 + 1.2842518958703766 * t174 * t473 * t1027 * t1031 + t8427 - 0.21309037037037037 * t174 * t1124 * t364 * t372 + 0.043374323531126094 * t174 * t3020 * t1023 - 0.06849333333333334 * t174 * t966 * t3059 - 141.7218633942076 * t174 * t155 * t3063 * t3068 - 0.41096 * t174 * t3038 * t3071 + 13.218398198777741 * t174 * t155 * t3075 * t3077 + 623.3672123775311 * t3081 * t8171 * t1030 - 24829.60425438716 * t62 / t987 / t970 * t8564 * t3067 - t8473 + t8477 + t8505;
    (t8686,)
}
