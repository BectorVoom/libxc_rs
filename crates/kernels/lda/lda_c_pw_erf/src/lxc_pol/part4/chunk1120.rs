//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1120/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1120<F: Float>(t15782: F, t16378: F, t557: F, t2420: F, t925: F, t2412: F, t13643: F, t15728: F, t15823: F, t1210: F, t191: F, t21: F, t13653: F, t13657: F, t10090: F, t13631: F, t13635: F, t13639: F, t13661: F, t13663: F) -> (F, F, F, F, F, F, F, F) {
    let t16395 = t15782 * t557 * t16378;
    let t16397 = t925 * t2420;
    let t16399 = t925 * t2412;
    let t16402 = t15823 * t13643 * t15728;
    let t16405 = t21 * t1210 * t191;
    let t16410 = t15823 * t13653 * t15728;
    let t16416 = t15823 * t13657 * t15728;
    let t16424 = 0.5758666666666666 * t16395 - 0.015996296296296297 * t16397 - 0.010664197530864198 * t16399 + 1.7276 * t16402 - 0.10666666666666667 * t16405 * t13635 * t15728 - 1.1517333333333333 * t16410 + 0.023703703703703703 * t16405 * t13639 * t15728 + 0.31992592592592595 * t16416 + 0.32 * t16405 * t13631 * t15728 + 0.07464938271604939 * t10090 + 0.05925925925925926 * t13661 - 0.009876543209876543 * t13663;
    (t16395, t16397, t16399, t16402, t16405, t16410, t16416, t16424)
}
