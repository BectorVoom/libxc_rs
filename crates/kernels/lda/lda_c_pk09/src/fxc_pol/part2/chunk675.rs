//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 675/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk675<F: Float>(t2181: F, t4655: F, t119: F, t121: F, t787: F, t120: F, t1062: F, t2182: F, t721: F, t2166: F, t3423: F) -> (F, F, F, F, F) {
    let t7783 = t2181 * t4655;
    let t7784 = t7783 * t119;
    let t7785 = t121 * t787;
    let t7786 = t120 * t7785;
    let t7789 = t2182 * t1062;
    let t7790 = t7789 * t721;
    let t7792 = t2182 * t119;
    let t7795 = t3423 * t2166;
    (t7784, t7786, t7790, t7792, t7795)
}
