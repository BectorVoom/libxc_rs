//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 969/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk969<F: Float>(t20225: F, t12832: F, t20209: F, t20210: F, t20211: F, t20213: F, t20215: F, t20219: F, t20221: F, t20222: F, t20224: F, t9770: F, t161: F, t489: F, t7807: F, t1915: F, t19349: F, t493: F) -> (F, F, F, F) {
    let t20226 = t20225 / 15.0;
    let t20227 = -t12832 + t20209 + t20210 - t20211 - t9770 - t20213 + t20215 + t20219 - t20221 - t20222 + t20224 + t20226;
    let t20234 = t161 * t489 * t7807;
    let t20235 = 2.0 / 15.0 * t20234;
    let t20238 = 2.0 / 45.0 * t493 * t1915 * t19349;
    (t20226, t20227, t20235, t20238)
}
