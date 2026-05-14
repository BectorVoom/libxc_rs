//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 926/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk926<F: Float>(t19475: F, t2010: F, t5260: F, t15764: F, t15770: F, t15772: F, t15774: F, t19458: F, t19461: F, t19463: F, t19466: F, t19469: F, t19474: F, t1444: F, t7513: F, t493: F, t5447: F, t7512: F) -> (F, F, F, F, F, F, F, F) {
    let t19478 = 16.0 / 27.0 * t2010 * t5260 * t19475;
    let t19479 = 4.0 / 45.0 * t15764;
    let t19480 = 4.0 / 15.0 * t15770;
    let t19481 = 4.0 / 9.0 * t15772;
    let t19482 = 16.0 / 45.0 * t15774;
    let t19483 = -t19458 + t19461 + t19463 + t19466 + t19469 + t19474 + t19478 + t19479 + t19480 - t19481 - t19482;
    let t19485 = 2.0 / 15.0 * t1444 * t7513;
    let t19488 = 2.0 / 15.0 * t493 * t5447 * t7512;
    (t19478, t19479, t19480, t19481, t19482, t19483, t19485, t19488)
}
