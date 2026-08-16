//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 574/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk574<F: Float>(t1392: F, t432: F, t1397: F, t1396: F, t435: F, t132: F, t1512: F, t479: F, t1490: F, t489: F, t161: F, t1541: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t3064 = t432 * t1392;
    let t3065 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t3064;
    let t3067 = t432 * t1397 / F::cast_from(5.0_f64);
    let t3068 = t435 * t1396;
    let t3069 = t132 * t3068;
    let t3070 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t3069;
    let t3072 = t1512 * t479 / F::cast_from(10.0_f64);
    let t3073 = t489 * t1490;
    let t3074 = t161 * t3073;
    let t3075 = t3074 / F::cast_from(15.0_f64);
    let t3076 = t435 * t1541;
    (t3064, t3065, t3067, t3068, t3069, t3070, t3072, t3073, t3074, t3075, t3076)
}
