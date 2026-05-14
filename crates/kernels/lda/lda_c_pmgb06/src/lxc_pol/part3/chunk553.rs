//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 553/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk553<F: Float>(t1392: F, t432: F, t1397: F, t1396: F, t435: F, t132: F, t1512: F, t479: F, t1490: F, t489: F, t161: F, t1541: F, t134: F, t138: F, t2897: F, t455: F, t947: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t3064 = t432 * t1392;
    let t3065 = 2.0 / 15.0 * t3064;
    let t3067 = t432 * t1397 / 5.0;
    let t3068 = t435 * t1396;
    let t3069 = t132 * t3068;
    let t3070 = 2.0 / 15.0 * t3069;
    let t3072 = t1512 * t479 / 10.0;
    let t3073 = t489 * t1490;
    let t3074 = t161 * t3073;
    let t3075 = t3074 / 15.0;
    let t3076 = t435 * t1541;
    let t3077 = t132 * t3076;
    let t3078 = t3077 / 15.0;
    let t3080 = t138 * t2897 * t134;
    let t3081 = 0.005877407407407408 * t3080;
    let t3082 = t947 * t455;
    (t3064, t3065, t3067, t3068, t3069, t3070, t3072, t3073, t3074, t3075, t3076, t3077, t3078, t3080, t3081, t3082)
}
