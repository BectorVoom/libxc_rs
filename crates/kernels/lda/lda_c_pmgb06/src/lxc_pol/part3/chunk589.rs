//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 589/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk589<F: Float>(t3198: F, t500: F, t1417: F, t1447: F, t1465: F, t2912: F, t496: F, t493: F, t3164: F, t3166: F, t3168: F, t3171: F, t3176: F, t3179: F, t3181: F, t3183: F, t3185: F, t3188: F, t3193: F, t3197: F) -> (F, F, F, F, F, F, F) {
    let t3200 = t3198 * t500 / F::new(15.0);
    let t3201 = t1447 * t1417;
    let t3202 = F::new(4.0) / F::new(45.0) * t3201;
    let t3203 = t1465 * t2912;
    let t3204 = t496 * t3203;
    let t3206 = F::new(2.0) / F::new(15.0) * t493 * t3204;
    let t3207 = t3164 + t3166 - t3168 - t3171 - t3176 + t3179 + t3181 + t3183 - t3185 - t3188 - t3193 + t3197 + t3200 - t3202 + t3206;
    (t3200, t3201, t3202, t3203, t3204, t3206, t3207)
}
