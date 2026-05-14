//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 811/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk811<F: Float>(t3055: F, t432: F, t132: F, t1396: F, t1547: F, t2943: F, t517: F, t3059: F, t435: F, t1595: F, t1499: F, t1636: F, t2880: F, t486: F, t161: F, t3460: F, t489: F) -> (F, F, F, F, F, F, F, F) {
    let t9598 = t432 * t3055;
    let t9601 = t132 * t1547 * t1396;
    let t9603 = t2943 * t517;
    let t9616 = t132 * t435 * t3059;
    let t9619 = t132 * t1547 * t1595;
    let t9626 = t1499 * t1636;
    let t9628 = t486 * t2880;
    let t9633 = t161 * t489 * t3460;
    (t9598, t9601, t9603, t9616, t9619, t9626, t9628, t9633)
}
