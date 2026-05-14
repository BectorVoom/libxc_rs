//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1021/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1021<F: Float>(t3883: F, t519: F, t7484: F, t2171: F, t6682: F, t12685: F, t12709: F, t21336: F, t21338: F, t21342: F, t21344: F, t21346: F, t21349: F, t21351: F, t21353: F, t21355: F) -> (F, F, F) {
    let t21357 = t519 * t3883 * t7484;
    let t21358 = 16.0 / 27.0 * t21357;
    let t21359 = t2171 * t6682;
    let t21360 = 16.0 / 45.0 * t21359;
    let t21361 = -t12685 - t12709 + 2.0 / 9.0 * t21336 + t21338 + t21342 - t21344 + t21346 + t21349 + t21351 + t21353 - t21355 - t21358 - t21360;
    (t21358, t21360, t21361)
}
