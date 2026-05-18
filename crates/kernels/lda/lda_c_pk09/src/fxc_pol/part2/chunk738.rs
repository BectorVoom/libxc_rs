//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 738/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk738<F: Float>(t119: F, t7633: F, t121: F, t861: F, t120: F, t1062: F, t2270: F, t721: F, t168: F, t2143: F, t609: F, t4030: F) -> (F, F, F, F, F, F) {
    let t7634 = t7633 * t119;
    let t7635 = t121 * t861;
    let t7636 = t120 * t7635;
    let t7639 = t2270 * t1062;
    let t7640 = t7639 * t721;
    let t7642 = t2270 * t119;
    let t7647 = t168 * t2143;
    let t7648 = t7647 * t609;
    let t7649 = t121 * t7648;
    let t7650 = t4030 * t7649;
    (t7634, t7636, t7640, t7642, t7647, t7650)
}
