//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 914/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk914<F: Float>(t15481: F, t2002: F, t6124: F, t432: F, t7503: F, t1447: F, t7535: F, t2497: F, t5194: F, t16513: F, t1893: F, t439: F, t1972: F, t6533: F, t6131: F, t6268: F, t6536: F) -> (F, F, F, F, F, F, F, F, F) {
    let t19278 = 2.0 / 15.0 * t15481;
    let t19280 = t2002 * t6124 / 15.0;
    let t19282 = t432 * t7503 / 30.0;
    let t19283 = t1447 * t7535;
    let t19284 = 4.0 / 45.0 * t19283;
    let t19285 = t5194 * t2497;
    let t19286 = 4.0 / 45.0 * t19285;
    let t19289 = t439 * t16513 * t1893 / 15.0;
    let t19291 = 2.0 / 15.0 * t1972 * t6533;
    let t19293 = t1972 * t6131 / 15.0;
    let t19295 = 4.0 / 15.0 * t6268 * t6536;
    (t19278, t19280, t19282, t19284, t19286, t19289, t19291, t19293, t19295)
}
