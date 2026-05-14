//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 707/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk707<F: Float>(t2012: F, t5168: F, t3216: F, t805: F, t439: F, t1600: F, t2088: F, t529: F, t1992: F, t493: F, t165: F, t511: F, t1994: F, t1444: F, t1995: F, t1447: F, t1989: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t5170 = 4.0 / 45.0 * t5168 * t2012;
    let t5171 = t3216 * t805;
    let t5173 = t439 * t5171 / 45.0;
    let t5174 = t1600 * t2088;
    let t5175 = t5174 * t529;
    let t5176 = t1992 * t5175;
    let t5178 = 2.0 / 15.0 * t493 * t5176;
    let t5179 = t165 * t511;
    let t5180 = t5179 * t1994;
    let t5182 = 2.0 / 15.0 * t493 * t5180;
    let t5184 = 2.0 / 15.0 * t1444 * t1995;
    let t5186 = 4.0 / 135.0 * t1447 * t1989;
    (t5170, t5171, t5173, t5174, t5175, t5176, t5178, t5179, t5180, t5182, t5184, t5186)
}
