//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 804/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk804<F: Float>(t3153: F, t357: F, t40: F, t174: F, t2749: F, t936: F, t1124: F, t318: F, t335: F, t1022: F, t1010: F, t386: F, t400: F, t2946: F, t8171: F, t1059: F, t2942: F) -> (F, F, F, F, F, F, F) {
    let t8419 = t40 * t357 * t3153;
    let t8423 = 0.14246666666666666 * t174 * t2749 * t936;
    let t8427 = 0.22161481481481482 * t174 * t1124 * t318 * t335;
    let t8428 = t1022 * t1022;
    let t8432 = 3.5089340384731225 * t400 * t1010 * t8428 * t386;
    let t8437 = 14.03573615389249 * t400 * t2946 * t8171 * t386;
    let t8438 = t1059 * t2942;
    (t8419, t8423, t8427, t8428, t8432, t8437, t8438)
}
