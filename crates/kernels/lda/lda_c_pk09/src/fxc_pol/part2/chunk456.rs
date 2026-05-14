//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 456/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk456<F: Float>(t1685: F, t1736: F, t2733: F, t2736: F, t1681: F, t1745: F, t132: F, t2730: F, t93: F) -> (F, F, F, F, F) {
    let t2738 = t1685 - 1.5625 * t2733 + t1736 + 1.5625 * t2736;
    let t2739 = t1681 * t2738;
    let t2740 = t2739 * t1745;
    let t2743 = t132 * t2730;
    let t2744 = t93 * t2743;
    (t2738, t2739, t2740, t2743, t2744)
}
