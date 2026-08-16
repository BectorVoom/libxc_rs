//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 583/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk583<F: Float>(t3149: F, t1517: F, t432: F, t1504: F, t486: F, t1554: F, t512: F, t161: F, t1499: F, t490: F, t3065: F, t3067: F, t3070: F, t3072: F, t3075: F, t3078: F, t3124: F, t3126: F, t3136: F, t3138: F, t3148: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t3150 = t3149 / F::cast_from(15.0_f64);
    let t3151 = t432 * t1517;
    let t3152 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t3151;
    let t3153 = t486 * t1504;
    let t3154 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t3153;
    let t3155 = t1554 * t512;
    let t3156 = t161 * t3155;
    let t3157 = t3156 / F::cast_from(45.0_f64);
    let t3158 = t1499 * t490;
    let t3159 = t3158 / F::cast_from(15.0_f64);
    let t3160 = -t3065 - t3067 - t3070 - t3072 + t3075 + t3078 + t3124 + t3126 + t3136 + t3138 + t3148 + t3150 + t3152 + t3154 - t3157 + t3159;
    (t3150, t3151, t3152, t3153, t3154, t3155, t3156, t3157, t3158, t3159, t3160)
}
