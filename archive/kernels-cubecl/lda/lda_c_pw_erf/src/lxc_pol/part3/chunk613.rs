//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 613/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk613<F: Float>(t1247: F, t325: F, t1458: F, t56: F, t3519: F, t11: F, t1124: F, t174: F, t177: F, t25: F, t3508: F, t3510: F, t3512: F, t3520: F, t3524: F, t3528: F, t3530: F, t3532: F) -> (F, F, F, F, F, F, F, F) {
    let t3534 = t325 * t1247;
    let t3536 = t56 * t1458;
    let t3537 = t3536 * t3519;
    let t3538 = t11 * t3537;
    let t3540 = t1124 * t56;
    let t3542 = t174 * t3540 * t177;
    let t3543 = F::cast_from(0.11197407407407407_f64) * t3542;
    let t3544 = -F::cast_from(0.022222222222222223_f64) * t3508 + F::cast_from(0.013333333333333334_f64) * t3510 + F::cast_from(0.0044444444444444444_f64) * t3512 - F::cast_from(0.002962962962962963_f64) * t25 * t3520 - F::cast_from(0.006666666666666667_f64) * t25 * t3524 - F::cast_from(0.035991666666666665_f64) * t3528 - F::cast_from(0.047988888888888886_f64) * t3530 + F::cast_from(0.035991666666666665_f64) * t3532 + F::cast_from(0.023994444444444443_f64) * t3534 - F::cast_from(0.03999074074074074_f64) * t3538 - t3543;
    (t3534, t3536, t3537, t3538, t3540, t3542, t3543, t3544)
}
