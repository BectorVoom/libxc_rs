//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 320/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk320<F: Float>(t1143: F, t169: F, t289: F, t274: F, t39: F, t462: F, t678: F, t147: F) -> (F, F, F, F) {
    let t1146 = F::cast_from(0.031835665774679375_f64) * t169 * t289 * t1143;
    let t1148 = F::cast_from(0.31995040645307626_f64) * t39 * t274;
    let t1149 = t462 * t678;
    let t1155 = t39 * t147;
    (t1146, t1148, t1149, t1155)
}
