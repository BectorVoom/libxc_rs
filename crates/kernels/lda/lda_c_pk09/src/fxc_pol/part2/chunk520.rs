//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 520/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk520<F: Float>(t3130: F, t748: F, t874: F, t340: F, t712: F, t93: F, t94: F) -> (F, F, F, F) {
    let t3131 = F::cast_from(4.800217903952168_f64) * t3130;
    let t3132 = t748 * t874;
    let t3138 = t340 * t712;
    let t3141 = t93 * t94;
    (t3131, t3132, t3138, t3141)
}
