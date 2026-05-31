//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 763/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk763<F: Float>(t1960: F, t595: F, t1982: F, t544: F, t1498: F, t835: F, t2100: F, t511: F, t558: F, t588: F, t1371: F, t4671: F) -> (F, F, F, F, F, F) {
    let t4966 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t1960 * t595;
    let t4968 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t1982 * t544;
    let t4970 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t1498 * t835;
    let t4972 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t511 * t2100;
    let t4981 = t588 * t558;
    let t4988 = t1371 * t4671;
    (t4966, t4968, t4970, t4972, t4981, t4988)
}
