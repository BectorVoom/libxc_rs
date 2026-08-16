//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1144/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1144<F: Float>(t2131: F, t3455: F, t2127: F, t5065: F, t4703: F, t568: F, t10294: F, t5031: F, t565: F, t548: F, t9933: F, t10296: F) -> (F, F, F, F, F, F, F, F) {
    let t13396 = F::cast_from(4.0_f64) / F::cast_from(5.0_f64) * t3455 * t2131;
    let t13397 = t5065 * t2127;
    let t13398 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t13397;
    let t13400 = F::cast_from(4.0_f64) / F::cast_from(5.0_f64) * t5065 * t2131;
    let t13401 = t4703 * t568;
    let t13402 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t13401;
    let t13403 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t10294;
    let t13405 = F::cast_from(2.0_f64) / F::cast_from(5.0_f64) * t565 * t5031;
    let t13407 = F::cast_from(4.0_f64) / F::cast_from(5.0_f64) * t548 * t9933;
    let t13408 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t10296;
    (t13396, t13398, t13400, t13402, t13403, t13405, t13407, t13408)
}
