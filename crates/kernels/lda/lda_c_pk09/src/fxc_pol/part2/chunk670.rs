//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 670/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk670<F: Float>(t309: F, t454: F, t6240: F, t1680: F, t520: F, t1665: F, t1747: F) -> (F, F, F) {
    let t6242 = t309 * t454 * t6240;
    let t6247 = F::cast_from(1.0_f64) / t1680 / t520;
    let t6253 = t1747 * t1665;
    (t6242, t6247, t6253)
}
