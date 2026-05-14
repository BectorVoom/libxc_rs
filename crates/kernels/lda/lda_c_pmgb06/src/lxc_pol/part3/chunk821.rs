//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 821/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk821<F: Float>(t1450: F, t1600: F, t1423: F, t2971: F, t3303: F, t2962: F, t3276: F, t3280: F, t135: F, t1438: F, t144: F, t1461: F, t1489: F, t3247: F, t511: F, t1464: F, t164: F, t170: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t10152 = t1450 * t1600;
    let t10156 = t1423 * t2971;
    let t10158 = t1423 * t3303;
    let t10161 = t1423 * t2962;
    let t10190 = t1423 * t3276;
    let t10196 = t1423 * t3280;
    let t10203 = 1.0 / t135 / t1438 * t144;
    let t10216 = t1461 * t1489;
    let t10220 = t3247 * t511;
    let t10230 = 1.0 / t164 / t1464 * t170;
    (t10152, t10156, t10158, t10161, t10190, t10196, t10203, t10216, t10220, t10230)
}
