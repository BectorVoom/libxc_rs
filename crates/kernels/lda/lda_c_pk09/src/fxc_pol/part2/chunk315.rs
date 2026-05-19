//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 315/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk315<F: Float>(t142: F, t1430: F, t408: F, t129: F, t94: F) -> (F, F, F) {
    let t1431 = t142 * t1430;
    let t1433 = F::cast_from(2.3693919160612835_f64) * t408 * t1431;
    let t1434 = t129 * t94;
    (t1431, t1433, t1434)
}
