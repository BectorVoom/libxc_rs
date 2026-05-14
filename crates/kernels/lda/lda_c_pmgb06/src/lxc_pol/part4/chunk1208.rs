//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1208/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1208<F: Float>(t16199: F, t16201: F, t16204: F, t16207: F, t16210: F, t16212: F, t16214: F, t16216: F, t16218: F, t16220: F, t16228: F, t9457: F, t9461: F, t9467: F, t9470: F, t12224: F, t12227: F, t16237: F, t16239: F, t16242: F, t16243: F, t16244: F, t16248: F, t16250: F, t16252: F, t16255: F, t16259: F, t9478: F, t9481: F, t9483: F) -> (F, F) {
    let t18214 = -t16199 - t16201 - t16204 - t16207 - t16210 - t16212 - t16214 - t16216 + t16218 - t16220 + t16228 + 0.003030876351851852 * t9457 + t9461 + t9467 + 2.0 / 3.0 * t9470;
    let t18218 = t9478 + t9481 + 0.36466666666666664 * t9483 - 8.0 / 27.0 * t12224 - 4.0 / 9.0 * t12227 + t16237 + t16239 + t16242 + t16243 + t16244 - t16248 + t16250 + t16252 + t16255 + t16259;
    (t18214, t18218)
}
