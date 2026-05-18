//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 888/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk888<F: Float>(t1073: F, t3007: F, t1184: F, t119: F, t395: F, t84: F, t174: F, t473: F, t903: F, t908: F, t912: F, t914: F) -> (F, F, F, F) {
    let t8464 = t1073 * t3007;
    let t8469 = F::new(0.0018989760778855128) * t395 * t119 * t1184 * t84;
    let t8473 = F::new(2.291123905095794) * t174 * t473 * t903 * t908;
    let t8477 = F::new(0.2849333333333333) * t174 * t473 * t912 * t914;
    (t8464, t8469, t8473, t8477)
}
