//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 645/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk645<F: Float>(t1450: F, t5569: F, t1369: F, t5308: F, t1240: F, t1350: F, t1349: F, t1337: F, t348: F, t4767: F, t1279: F, t1336: F) -> (F, F, F, F, F) {
    let t5570 = t1450 * t5569;
    let t5572 = t1369 * t5308;
    let t5574 = t1350 * t1240;
    let t5575 = t1349 * t5574;
    let t5576 = t1337 * t5575;
    let t5579 = F::cast_from(1.6715885419444727_f64) * t348 * t4767;
    let t5584 = t1279 * t1336;
    (t5570, t5572, t5576, t5579, t5584)
}
