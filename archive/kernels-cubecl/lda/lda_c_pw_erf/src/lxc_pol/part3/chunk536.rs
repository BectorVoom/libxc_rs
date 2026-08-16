//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 536/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk536<F: Float>(t2719: F, t918: F, t1124: F, t119: F, t321: F, t11: F, t2: F, t39: F, t928: F, t328: F, t1953: F, t2061: F, t2717: F) -> (F, F, F, F, F, F, F) {
    let t2720 = t918 * t2719;
    let t2722 = t119 * t1124;
    let t2723 = t321 * t2722;
    let t2726 = F::cast_from(1.0_f64)/pow_3_2::<F>(t11);
    let t2727 = t2726 * t2;
    let t2728 = t2727 * t39;
    let t2730 = t928 * t2719;
    let t2732 = t328 * t2722;
    let t2735 = -F::cast_from(3.4523333333333333_f64) * t2717 + F::cast_from(2.3015555555555554_f64) * t2720 - F::cast_from(2.6851481481481483_f64) * t2723 - F::cast_from(0.9393222222222222_f64) * t1953 + F::cast_from(0.073355_f64) * t2728 - F::cast_from(0.14671_f64) * t2730 - F::cast_from(0.17116166666666666_f64) * t2732 - F::cast_from(0.36793333333333333_f64) * t2061;
    (t2720, t2723, t2727, t2728, t2730, t2732, t2735)
}
