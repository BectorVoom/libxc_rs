//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1147/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1147<F: Float>(t10046: F, t13230: F, t13232: F, t13235: F, t13237: F, t13239: F, t13241: F, t17012: F, t17013: F, t17014: F, t17015: F, t17017: F, t17018: F, t17020: F, t17252: F, t13243: F) -> (F, F, F, F, F, F, F, F, F) {
    let t17253 = t10046 / 135.0;
    let t17254 = 2.0 / 45.0 * t13230;
    let t17255 = 4.0 / 45.0 * t13232;
    let t17256 = 2.0 / 45.0 * t13235;
    let t17257 = 2.0 / 45.0 * t13237;
    let t17258 = 4.0 / 45.0 * t13239;
    let t17259 = 8.0 / 135.0 * t13241;
    let t17260 = t17012 + t17013 + t17014 + t17015 + t17017 + t17018 + t17020 - t17252 + t17253 + t17254 + t17255 + t17256 + t17257 + t17258 + t17259;
    let t17261 = 8.0 / 243.0 * t13243;
    (t17253, t17254, t17255, t17256, t17257, t17258, t17259, t17260, t17261)
}
