//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1108/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1108<F: Float>(t1981: F, t835: F, t1454: F, t493: F, t5312: F, t1461: F, t1835: F, t1466: F, t1989: F, t3198: F, t1444: F, t4585: F) -> (F, F, F, F, F) {
    let t13177 = t1981 * t835;
    let t13178 = F::cast_from(8.0_f64) / F::cast_from(1215.0_f64) * t13177;
    let t13181 = t493 * t5312 * t1454 / F::cast_from(15.0_f64);
    let t13182 = t1461 * t1835;
    let t13185 = t493 * t13182 * t1466 / F::cast_from(9.0_f64);
    let t13187 = t3198 * t1989 / F::cast_from(15.0_f64);
    let t13189 = t1444 * t4585 / F::cast_from(15.0_f64);
    (t13178, t13181, t13185, t13187, t13189)
}
