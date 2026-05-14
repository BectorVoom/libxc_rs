//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 611/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk611<F: Float>(t144: F, t3031: F, t1600: F, t511: F, t1603: F, t489: F, t161: F, t1630: F, t435: F, t132: F, t1547: F, t478: F, t1540: F, t464: F, t1392: F, t432: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t3032 = t144 * t3031;
    let t3038 = t511 * t1600;
    let t3043 = t489 * t1603;
    let t3044 = t161 * t3043;
    let t3050 = t435 * t1630;
    let t3051 = t132 * t3050;
    let t3055 = t1547 * t478;
    let t3056 = t132 * t3055;
    let t3058 = t1540 * t464;
    let t3064 = t432 * t1392;
    (t3032, t3038, t3043, t3044, t3050, t3051, t3055, t3056, t3058, t3064)
}
