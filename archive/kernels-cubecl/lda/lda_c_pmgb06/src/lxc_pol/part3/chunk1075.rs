//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1075/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1075<F: Float>(t464: F, t4779: F, t1386: F, t439: F, t1924: F, t493: F, t9925: F, t1385: F, t332: F, t443: F, t5039: F, t1387: F, t5220: F) -> (F, F, F, F) {
    let t12772 = t4779 * t464;
    let t12775 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t439 * t12772 * t1386;
    let t12778 = t493 * t9925 * t1924 / F::cast_from(15.0_f64);
    let t12783 = t439 * t1385 * t5039 * t443 * t332 / F::cast_from(15.0_f64);
    let t12784 = t5220 * t1387;
    (t12775, t12778, t12783, t12784)
}
