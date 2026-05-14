//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 798/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk798<F: Float>(t1039: F, t1067: F, t1035: F, t1070: F, t1037: F, t325: F, t333: F, t903: F, t907: F, t935: F, t912: F, t936: F, t38: F, t36: F, t88: F, t3165: F, t338: F) -> (F, F, F, F, F, F, F, F) {
    let t8495 = t1067 * t1039;
    let t8497 = t1070 * t1035;
    let t8499 = t1067 * t1037;
    let t8505 = 3.436685857643691 * t325 * t903 * t935 * t907 * t333;
    let t8509 = 0.4274 * t325 * t912 * t333 * t936;
    let t8510 = t1070 * t1039;
    let t8512 = t38 * t38;
    let t8516 = 840.0 * t36 / t8512 * t88;
    let t8518 = t338 * t3165 * t88;
    (t8495, t8497, t8499, t8505, t8509, t8510, t8516, t8518)
}
