//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1179/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1179<F: Float>(t2193: F, t9752: F, t2171: F, t3780: F, t1480: F, t5334: F, t1488: F, t4804: F, t5378: F, t5382: F, t4753: F, t4943: F) -> (F, F, F, F, F, F, F) {
    let t13899 = F::cast_from(4.0_f64) / F::cast_from(5.0_f64) * t9752 * t2193;
    let t13901 = F::cast_from(4.0_f64) / F::cast_from(5.0_f64) * t2171 * t3780;
    let t13903 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t5334 * t1480;
    let t13905 = F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t5334 * t1488;
    let t13906 = t4804 * t5378;
    let t13907 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t13906;
    let t13909 = F::cast_from(4.0_f64) / F::cast_from(5.0_f64) * t4804 * t5382;
    let t13911 = F::cast_from(8.0_f64) / F::cast_from(5.0_f64) * t4753 * t4943;
    (t13899, t13901, t13903, t13905, t13907, t13909, t13911)
}
