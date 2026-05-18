//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 963/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk963<F: Float>(t1461: F, t2553: F, t350: F, t6186: F, t4641: F, t6190: F, t6161: F, t6166: F, t6176: F, t6179: F, t6182: F, t2579: F, t947: F) -> (F, F, F, F, F, F, F, F, F) {
    let t15299 = t1461 * t2553;
    let t15391 = t350 * t6186;
    let t15393 = t4641 * t6190;
    let t15399 = t350 * t6161;
    let t15401 = t350 * t6166;
    let t15403 = t350 * t6176;
    let t15405 = t350 * t6179;
    let t15407 = t4641 * t6182;
    let t15416 = t947 * t2579;
    (t15299, t15391, t15393, t15399, t15401, t15403, t15405, t15407, t15416)
}
