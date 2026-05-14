//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 610/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk610<F: Float>(t177: F, t3004: F, t161: F, t1423: F, t1560: F, t1166: F, t539: F, t188: F, t1830: F, t2060: F, t83: F, t1409: F, t398: F, t463: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t3005 = t3004 * t177;
    let t3007 = 4.0 / 405.0 * t161 * t3005;
    let t3008 = t1423 * t1560;
    let t3018 = t1166 * t539;
    let t3019 = t3018 * t188;
    let t3023 = 1.2833333333333334 * t1830 - 20.0 / 27.0 * t2060;
    let t3024 = t83 * t3023;
    let t3026 = 4.0 / 3.0 * t3024 * t188;
    let t3027 = t398 * t1409;
    let t3028 = t3027 * t188;
    let t3030 = t463 * t463;
    let t3031 = 1.0 / t3030;
    (t3005, t3007, t3008, t3018, t3019, t3023, t3024, t3026, t3027, t3028, t3030, t3031)
}
