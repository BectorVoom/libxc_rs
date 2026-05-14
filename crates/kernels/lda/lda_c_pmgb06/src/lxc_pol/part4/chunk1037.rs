//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1037/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1037<F: Float>(t15395: F, t36: F, t453: F, t350: F, t6161: F, t6166: F, t6176: F, t6179: F, t4641: F, t6182: F, t15375: F, t15380: F, t15384: F, t15389: F, t15391: F, t15393: F) -> (F, F, F, F, F, F, F) {
    let t15397 = t36 * t453 * t15395;
    let t15399 = t350 * t6161;
    let t15401 = t350 * t6166;
    let t15403 = t350 * t6176;
    let t15405 = t350 * t6179;
    let t15407 = t4641 * t6182;
    let t15409 = -0.007556666666666666 * t15375 - 0.02518888888888889 * t15380 + 0.002099074074074074 * t15384 + 0.005597530864197531 * t15389 - 0.007556666666666666 * t15391 + 0.05541555555555556 * t15393 + 0.011335 * t15397 + 0.002518888888888889 * t15399 - 0.0008396296296296296 * t15401 + 0.005037777777777778 * t15403 - 0.0013993827160493828 * t15405 - 0.01847185185185185 * t15407;
    (t15397, t15399, t15401, t15403, t15405, t15407, t15409)
}
