//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 296/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk296<F: Float>(t1287: F, t1311: F, t323: F, t337: F, t1284: F) -> (F, F, F) {
    let t1313 = F::cast_from(1.8805371096875316_f64) * t1311 * t1287;
    let t1314 = t323 * t337;
    let t1315 = t1314 * t1284;
    (t1313, t1314, t1315)
}
