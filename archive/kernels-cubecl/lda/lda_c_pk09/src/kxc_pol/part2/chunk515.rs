//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 515/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk515<F: Float>(t143: F, t2983: F, t569: F, t933: F, t17: F, t24: F, t580: F, t68: F, t1146: F, t228: F, t21: F, t12: F, t567: F) -> (F, F, F, F, F, F) {
    let t3032 = t143 * t2983;
    let t3034 = t933 * t569;
    let t3039 = t24 / t580 / t17 * t68;
    let t3040 = t228 * t1146;
    let t3041 = t3040 * t21;
    let t3044 = t12 * t567;
    (t3032, t3034, t3039, t3040, t3041, t3044)
}
