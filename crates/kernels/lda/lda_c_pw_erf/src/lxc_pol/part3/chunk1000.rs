//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1000/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1000<F: Float>(t2131: F, t3455: F, t2127: F, t5065: F, t4703: F, t568: F, t10294: F, t5031: F, t565: F, t548: F, t9933: F, t10296: F, t10311: F, t10315: F, t10318: F, t10320: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t13396 = 4.0 / 5.0 * t3455 * t2131;
    let t13397 = t5065 * t2127;
    let t13398 = 8.0 / 15.0 * t13397;
    let t13400 = 4.0 / 5.0 * t5065 * t2131;
    let t13401 = t4703 * t568;
    let t13402 = 4.0 / 15.0 * t13401;
    let t13403 = 8.0 / 15.0 * t10294;
    let t13405 = 2.0 / 5.0 * t565 * t5031;
    let t13407 = 4.0 / 5.0 * t548 * t9933;
    let t13408 = 16.0 / 45.0 * t10296;
    let t13409 = 8.0 / 135.0 * t10311;
    let t13410 = 8.0 / 81.0 * t10315;
    let t13411 = 16.0 / 27.0 * t10318;
    let t13412 = 8.0 / 45.0 * t10320;
    (t13396, t13398, t13400, t13402, t13403, t13405, t13407, t13408, t13409, t13410, t13411, t13412)
}
