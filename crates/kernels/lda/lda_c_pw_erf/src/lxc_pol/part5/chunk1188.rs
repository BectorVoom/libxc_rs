//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1188/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1188<F: Float>(t17434: F, t34: F, t473: F, t16144: F, t2479: F, t266: F, t17436: F, t2497: F, t806: F, t1325: F, t494: F, t5289: F) -> (F, F, F, F, F, F) {
    let t21576 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t17434;
    let t21577 = t34 * t473;
    let t21581 = F::cast_from(4.0_f64) / F::cast_from(5.0_f64) * t21577 * t16144 * t266 * t2479;
    let t21582 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t17436;
    let t21583 = t2497 * t806;
    let t21587 = F::cast_from(8.0_f64) / F::cast_from(5.0_f64) * t1325 * t5289 * t21583 * t494;
    (t21576, t21577, t21581, t21582, t21583, t21587)
}
