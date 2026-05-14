//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 643/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk643<F: Float>(t2571: F, t350: F, t2575: F, t2579: F, t2642: F, t405: F, t2645: F, t2639: F, t443: F, t5961: F, t473: F, t453: F, t36: F, t103: F, t2060: F, t6177: F, t6180: F, t6183: F, t6187: F, t6191: F, t6193: F, t6196: F, t6199: F, t6202: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t6205 = t350 * t2571;
    let t6207 = t350 * t2575;
    let t6209 = t350 * t2579;
    let t6211 = t405 * t2642;
    let t6213 = t405 * t2645;
    let t6215 = t405 * t2639;
    let t6217 = t443 * t5961;
    let t6218 = t473 * t6217;
    let t6221 = t453 * t6217;
    let t6222 = t36 * t6221;
    let t6224 = 0.14396666666666666 * t6177 - 0.03999074074074074 * t6180 - 0.09597777777777777 * t6183 - 0.21595 * t6187 + 0.2879333333333333 * t6191 - 0.04 * t103 * t6193 + 0.05333333333333334 * t2060 * t6196 + 0.013333333333333334 * t103 * t6199 - 0.0022222222222222222 * t103 * t6202 + 0.007998148148148148 * t6205 - 0.023994444444444443 * t6207 + 0.011997222222222222 * t6209 - 0.008888888888888889 * t6211 + 0.0044444444444444444 * t6213 + 0.0014814814814814814 * t6215 - 0.006666666666666667 * t103 * t6218 - 0.035991666666666665 * t6222;
    (t6205, t6207, t6209, t6211, t6213, t6215, t6217, t6218, t6221, t6222, t6224)
}
