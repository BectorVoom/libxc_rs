//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 917/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk917<F: Float>(t11567: F, t1773: F, t2266: F, t1183: F, t1798: F, t297: F, t301: F, t2789: F, t794: F, t1767: F, t1770: F, t419: F) -> (F, F, F, F, F) {
    let t11568 = F::new(0.15965645347006147) * t11567;
    let t11569 = t1773 * t2266;
    let t11600 = t297 * t1798 * t1183 * t301;
    let t11601 = F::new(0.03592270203076383) * t11600;
    let t11604 = t297 * t794 * t2789 * t301;
    let t11608 = t1767 * t1798 * t419 * t1770;
    (t11568, t11569, t11601, t11604, t11608)
}
