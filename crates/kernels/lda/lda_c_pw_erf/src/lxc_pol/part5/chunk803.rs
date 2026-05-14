//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 803/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk803<F: Float>(t174: F, t3149: F, t998: F, t155: F, t3127: F, t3131: F, t3135: F, t3137: F, t2745: F, t3123: F, t902: F, t906: F, t13: F, t8185: F, t3128: F, t907: F) -> (F, F, F, F, F, F) {
    let t8389 = 0.07123333333333333 * t174 * t998 * t3149;
    let t8393 = 36.84545214203136 * t174 * t155 * t3127 * t3131;
    let t8397 = 6.873371715287382 * t174 * t155 * t3135 * t3137;
    let t8400 = 0.4274 * t174 * t2745 * t3123;
    let t8407 = t902 * t902;
    let t8410 = t906 * t906;
    let t8414 = 24954.97798673547 * t13 / t8407 * t8185 / t8410;
    let t8417 = 578.9456755974397 * t3128 * t8185 * t907;
    (t8389, t8393, t8397, t8400, t8414, t8417)
}
