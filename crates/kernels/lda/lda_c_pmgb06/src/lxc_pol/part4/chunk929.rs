//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 929/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk929<F: Float>(t3216: F, t464: F, t122: F, t227: F, t8088: F, t107: F, t1126: F, t1180: F, t199: F, t2778: F, t4182: F, t610: F, t1669: F, t1735: F, t3993: F, t1135: F, t566: F) -> (F, F, F, F, F, F, F, F) {
    let t10412 = t3216 * t464;
    let t10472 = 0.9079060239445599 * t122 * t8088 * t227;
    let t10474 = t107 * t1180 * t1126;
    let t10479 = 2.0103076928521055 * t2778 * t199;
    let t10487 = t122 * t4182 * t610;
    let t10490 = t122 * t1669 * t1735;
    let t10492 = t3993 * t199;
    let t10494 = t1135 * t566;
    (t10412, t10472, t10474, t10479, t10487, t10490, t10492, t10494)
}
