//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 524/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk524<F: Float>(t3160: F, t3163: F, t741: F, t902: F, t609: F, t903: F, t904: F, t917: F, t891: F, t892: F, t896: F, t897: F) -> (F, F, F, F, F, F, F) {
    let t3165 = F::new(3.7610742193750633) * t3160 * t3163;
    let t3166 = t741 * t902;
    let t3172 = t903 * t904 * t609;
    let t3173 = t917 * t3172;
    let t3176 = t891 * t892 * t609;
    let t3177 = t917 * t3176;
    let t3190 = t896 * t897 * t609;
    (t3165, t3166, t3172, t3173, t3176, t3177, t3190)
}
