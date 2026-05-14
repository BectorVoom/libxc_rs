//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1037/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1037<F: Float>(t14043: F, t185: F, t3679: F, t795: F, t2070: F, t834: F, t211: F, t548: F, t812: F, t10632: F, t10643: F, t10656: F, t4589: F, t544: F, t14029: F, t14033: F, t14037: F, t14040: F, t14042: F) -> (F, F, F, F, F, F, F, F, F) {
    let t14044 = t185 * t14043;
    let t14045 = 16.0 / 405.0 * t14044;
    let t14047 = 4.0 / 5.0 * t795 * t3679;
    let t14048 = t2070 * t834;
    let t14049 = t211 * t14048;
    let t14050 = 16.0 / 405.0 * t14049;
    let t14052 = t548 * t2070 * t812;
    let t14053 = 32.0 / 405.0 * t14052;
    let t14054 = 8.0 / 27.0 * t10632;
    let t14055 = 32.0 / 45.0 * t10643;
    let t14056 = 32.0 / 135.0 * t10656;
    let t14058 = 4.0 / 5.0 * t4589 * t544;
    let t14059 = -t14029 - t14033 + t14037 - t14040 - t14042 - t14045 - t14047 - t14050 + t14053 + t14054 - t14055 - t14056 - t14058;
    (t14045, t14047, t14050, t14053, t14054, t14055, t14056, t14058, t14059)
}
