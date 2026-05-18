//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 691/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk691<F: Float>(t6522: F, t1686: F, t5153: F, t1240: F, t633: F, t6511: F) -> (F, F, F, F) {
    let t6523 = F::new(4.277978922036907) * t6522;
    let t6524 = t1686 * t5153;
    let t6525 = t1240 * t633;
    let t6527 = t6524 * t6511 * t6525;
    (t6523, t6524, t6525, t6527)
}
