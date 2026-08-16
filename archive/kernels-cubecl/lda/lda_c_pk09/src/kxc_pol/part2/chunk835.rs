//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 835/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk835<F: Float>(t8342: F, t890: F, t8065: F, t917: F, t1011: F, t161: F, t164: F, t2239: F, t4715: F, t4725: F, t7768: F, t7962: F, t8475: F, t8485: F, t8491: F, t8494: F, t8498: F, t8503: F, t8506: F, t8508: F) -> (F, F, F) {
    let t8510 = t890 * t8342;
    let t8512 = t917 * t8065;
    let t8514 = F::cast_from(0.04115066352984959_f64) * t164 * t8475 - F::cast_from(2.2140749178833072_f64) * t2239 * t1011 - F::cast_from(4.937333717448355_f64) * t161 * t7962 - F::cast_from(4.937333717448355_f64) * t161 * t7768 + F::cast_from(0.04115066352984959_f64) * t4725 * t8485 + F::cast_from(0.04115066352984959_f64) * t8491 + F::cast_from(0.04115066352984959_f64) * t4715 * t8494 + F::cast_from(0.04115066352984959_f64) * t4725 * t8498 + F::cast_from(0.04115066352984959_f64) * t4725 * t8503 + F::cast_from(1.8805371096875316_f64) * t8506 - F::cast_from(5.40024514194619_f64) * t8508 + F::cast_from(3.600163427964126_f64) * t8510 + F::cast_from(3.600163427964126_f64) * t8512;
    (t8510, t8512, t8514)
}
