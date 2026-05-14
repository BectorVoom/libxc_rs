//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 848/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk848<F: Float>(t10162: F, t10164: F, t10174: F, t1451: F, t2596: F, t5404: F, t5632: F, t5783: F, t9623: F, t9631: F, t9635: F, t9742: F, t9750: F, t10104: F, t68: F, t334: F) -> (F, F, F) {
    let t10177 = t10162 / 18.0 - t10164 * t1451 / 6.0 - t2596 * t5632 / 6.0 + 0.10237773105191754 * t9623 + 0.03412591035063918 * t9631 + 0.10237773105191754 * t9635 + 0.10237773105191754 * t9742 + 0.10237773105191754 * t9750 + t10174 / 18.0 + 0.04991874779241519 * t5404 - t5783;
    let t10181 = t10104 * t68;
    let t10182 = t10181 * t334;
    (t10177, t10181, t10182)
}
