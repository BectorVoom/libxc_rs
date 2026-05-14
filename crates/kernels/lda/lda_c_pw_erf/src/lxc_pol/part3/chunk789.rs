//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 789/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk789<F: Float>(t1030: F, t2983: F, t400: F, t8171: F, t1051: F, t2742: F, t1077: F, t3191: F, t344: F, t3148: F, t333: F, t904: F, t907: F, t1084: F, t474: F, t2704: F, t2710: F) -> (F, F, F, F, F, F, F, F) {
    let t8248 = 623.3672123775311 * t400 * t2983 * t8171 * t1030;
    let t8249 = t2742 * t1051;
    let t8251 = t2742 * t1077;
    let t8255 = t344 * t3191;
    let t8260 = 64.32729728860441 * t904 * t3148 * t907 * t333;
    let t8263 = 0.08674864706225219 * t1084 * t474 * t1077;
    let t8266 = 0.043374323531126094 * t1084 * t474 * t1051;
    let t8267 = t2704 * t2710;
    (t8248, t8249, t8251, t8255, t8260, t8263, t8266, t8267)
}
