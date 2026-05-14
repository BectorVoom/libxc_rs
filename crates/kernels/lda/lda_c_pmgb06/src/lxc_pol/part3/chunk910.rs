//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 910/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk910<F: Float>(t1680: F, t2022: F, t12219: F, t9457: F, t9461: F, t9467: F, t9470: F, t9478: F, t9481: F, t9483: F, t9491: F, t9494: F, t2026: F, t432: F, t4830: F, t132: F, t2851: F, t814: F) -> (F, F, F, F) {
    let t12224 = t2022 * t1680;
    let t12225 = 2.0 / 9.0 * t12224;
    let t12226 = -t12219 + 0.004546314527777778 * t9457 + t9461 + t9467 + t9470 + t9478 + t9481 + 0.547 * t9483 + t9491 / 3.0 + 0.06077777777777778 * t9494 - t12225;
    let t12227 = t2026 * t1680;
    let t12230 = 2.0 / 15.0 * t432 * t4830;
    let t12232 = t132 * t2851 * t814;
    (t12226, t12227, t12230, t12232)
}
