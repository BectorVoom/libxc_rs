//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1013/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1013<F: Float>(t1415: F, t1981: F, t337: F, t496: F, t1462: F, t1465: F, t1988: F, t3242: F, t493: F, t1992: F, t1993: F, t3382: F) -> (F, F, F, F) {
    let t12051 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t1981 * t496 * t1415 * t337;
    let t12055 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1981 * t1462 * t1465 * t337;
    let t12058 = t493 * t1988 * t3242 / F::cast_from(45.0_f64);
    let t12062 = t493 * t1992 * t1993 * t3382 / F::cast_from(15.0_f64);
    (t12051, t12055, t12058, t12062)
}
