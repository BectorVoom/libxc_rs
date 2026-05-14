//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 722/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk722<F: Float>(t1594: F, t5118: F, t137: F, t132: F, t1604: F, t831: F, t1392: F, t802: F, t1631: F, t3051: F, t3056: F, t3064: F, t4146: F, t4148: F, t4151: F, t5101: F, t5104: F, t5107: F, t5112: F, t5114: F, t5117: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t5119 = t5118 * t1594;
    let t5120 = t137 * t5119;
    let t5122 = t132 * t5120 / 15.0;
    let t5124 = t831 * t1604 / 15.0;
    let t5126 = 2.0 / 45.0 * t802 * t1392;
    let t5128 = t802 * t1631 / 30.0;
    let t5129 = t3051 / 45.0;
    let t5130 = 2.0 / 135.0 * t3056;
    let t5131 = 2.0 / 45.0 * t3064;
    let t5132 = -2.0 / 45.0 * t4146 + 4.0 / 135.0 * t4148 - t4151 + t5101 - t5104 - t5107 + t5112 - t5114 - t5117 + t5122 + t5124 - t5126 - t5128 - t5129 + t5130 - t5131;
    (t5119, t5120, t5122, t5124, t5126, t5128, t5129, t5130, t5131, t5132)
}
