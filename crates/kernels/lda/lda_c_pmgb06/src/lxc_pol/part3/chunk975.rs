//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 975/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk975<F: Float>(t13230: F, t432: F, t4810: F, t161: F, t489: F, t5416: F, t1499: F, t1933: F, t4790: F, t486: F, t1447: F, t5359: F, t1902: F, t3213: F, t5494: F, t1387: F, t5187: F) -> (F, F, F, F, F, F, F, F, F) {
    let t13231 = t13230 / 15.0;
    let t13232 = t432 * t4810;
    let t13233 = 2.0 / 15.0 * t13232;
    let t13235 = t161 * t489 * t5416;
    let t13236 = t13235 / 15.0;
    let t13237 = t1499 * t1933;
    let t13238 = t13237 / 15.0;
    let t13239 = t486 * t4790;
    let t13240 = 2.0 / 15.0 * t13239;
    let t13241 = t1447 * t5359;
    let t13242 = 4.0 / 45.0 * t13241;
    let t13243 = t3213 * t1902;
    let t13244 = 2.0 / 81.0 * t13243;
    let t13245 = t1447 * t5494;
    let t13246 = 4.0 / 45.0 * t13245;
    let t13248 = 2.0 / 15.0 * t5187 * t1387;
    (t13231, t13233, t13236, t13238, t13240, t13242, t13244, t13246, t13248)
}
