//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 999/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk999<F: Float>(t518: F, t5214: F, t3899: F, t5321: F, t571: F, t3663: F, t822: F, t1294: F, t1960: F, t108: F, t2075: F, t267: F) -> (F, F, F, F, F) {
    let t12299 = t5214 * t518;
    let t12307 = t571 * t3899 * t5321;
    let t12309 = t822 * t3663;
    let t12311 = t1960 * t1294;
    let t12314 = t2075 * t108 * t267;
    (t12299, t12307, t12309, t12311, t12314)
}
