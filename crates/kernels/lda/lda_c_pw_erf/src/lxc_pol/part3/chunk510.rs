//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 510/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk510<F: Float>(t2722: F, t321: F, t11: F, t2: F, t39: F, t2719: F, t928: F, t328: F, t1953: F, t2061: F, t2717: F, t2720: F) -> (F, F, F, F, F, F) {
    let t2723 = t321 * t2722;
    let t2726 = 1.0/pow_3_2(t11);
    let t2727 = t2726 * t2;
    let t2728 = t2727 * t39;
    let t2730 = t928 * t2719;
    let t2732 = t328 * t2722;
    let t2735 = -3.4523333333333333 * t2717 + 2.3015555555555554 * t2720 - 2.6851481481481483 * t2723 - 0.9393222222222222 * t1953 + 0.073355 * t2728 - 0.14671 * t2730 - 0.17116166666666666 * t2732 - 0.36793333333333333 * t2061;
    (t2723, t2727, t2728, t2730, t2732, t2735)
}
