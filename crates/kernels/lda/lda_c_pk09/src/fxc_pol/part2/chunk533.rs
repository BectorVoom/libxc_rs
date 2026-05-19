//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 533/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk533<F: Float>(t3290: F, t957: F, t3223: F, t944: F, t611: F, t625: F) -> (F, F, F, F) {
    let t3292 = F::cast_from(2.427516195194328_f64) * t957 * t3290;
    let t3300 = t944 * t3223;
    let t3303 = F::cast_from(1.8805371096875316_f64) * t944 * t3290;
    let t3317 = t611 * t625;
    (t3292, t3300, t3303, t3317)
}
