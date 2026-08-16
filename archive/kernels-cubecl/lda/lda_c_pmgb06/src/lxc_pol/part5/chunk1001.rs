//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1001/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1001<F: Float>(t588: F, t6721: F, t97: F, t607: F, t6355: F, t1710: F, t2519: F, t122: F, t569: F, t6913: F, t107: F, t1180: F, t2407: F) -> (F, F, F, F, F) {
    let t18284 = t6721 * t97 * t588;
    let t18329 = t6355 * t607;
    let t18331 = t2519 * t1710;
    let t18404 = t122 * t569 * t6913;
    let t18407 = t107 * t1180 * t2407;
    (t18284, t18329, t18331, t18404, t18407)
}
