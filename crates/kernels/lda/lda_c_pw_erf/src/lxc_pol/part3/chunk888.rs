//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 888/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk888<F: Float>(t1112: F, t159: F, t285: F, t39: F, t3309: F, t3310: F, t343: F, t3318: F, t3319: F, t1687: F, t5021: F, t1653: F) -> (F, F, F, F, F) {
    let t8845 = t39 * t1112 * t159 * t285;
    let t8862 = F::cast_from(2.6116266666666665_f64) * t3309 * t3310 * t343;
    let t8865 = F::cast_from(15.589466666666667_f64) * t3318 * t3319 * t343;
    let t8867 = F::cast_from(2.9018074074074076_f64) * t1687 * t5021;
    let t8869 = F::cast_from(5.773876543209877_f64) * t1653 * t5021;
    (t8845, t8862, t8865, t8867, t8869)
}
