//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 846/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk846<F: Float>(t1272: F, t4913: F, t1239: F, t1234: F, t315: F, t934: F, t1238: F, t64: F, t97: F, t342: F, t740: F, t3576: F) -> (F, F, F, F, F, F) {
    let t8293 = F::new(2.9018074074074076) * t1272 * t4913;
    let t8295 = F::new(5.773876543209877) * t1239 * t4913;
    let t8299 = t934 * t315 * t1234;
    let t8300 = t1238 * t64 * t97 * t8299;
    let t8305 = t934 * t740 * t342;
    let t8306 = t3576 * t8305;
    (t8293, t8295, t8299, t8300, t8305, t8306)
}
