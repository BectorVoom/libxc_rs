//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1171/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1171<F: Float>(t331: F, t6519: F, t5021: F, t6522: F, t6525: F, t6510: F, t6513: F, t1268: F, t15794: F, t15802: F, t15807: F, t15811: F, t15816: F, t15872: F, t15877: F, t25: F, t3516: F, t538: F, t9761: F) -> (F,) {
    let t17272 = t331 * t6519;
    let t17274 = t5021 * t6522;
    let t17288 = t331 * t6525;
    let t17290 = t331 * t6510;
    let t17295 = t331 * t6513;
    let t17297 = -0.0022222222222222222 * t25 * t1268 * t15872 - 0.002962962962962963 * t25 * t3516 * t15877 + 0.003950617283950617 * t17272 + 0.03851851851851852 * t17274 + 0.013333333333333334 * t25 * t1268 * t15802 + 0.035555555555555556 * t25 * t3516 * t15807 - 0.002962962962962963 * t25 * t3516 * t15811 - 0.006913580246913581 * t25 * t9761 * t15816 + 0.05333333333333334 * t17288 - 0.017777777777777778 * t17290 - 0.04 * t25 * t538 * t15794 + 0.002962962962962963 * t17295;
    (t17297,)
}
