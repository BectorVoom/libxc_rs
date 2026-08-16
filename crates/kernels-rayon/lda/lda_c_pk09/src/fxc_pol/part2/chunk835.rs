//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 835/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk835(t8342: f64, t890: f64, t8065: f64, t917: f64, t1011: f64, t161: f64, t164: f64, t2239: f64, t4715: f64, t4725: f64, t7768: f64, t7962: f64, t8475: f64, t8485: f64, t8491: f64, t8494: f64, t8498: f64, t8503: f64, t8506: f64, t8508: f64) -> (f64, f64, f64) {
    let t8510 = t890 * t8342;
    let t8512 = t917 * t8065;
    let t8514 = 0.04115066352984959_f64 * t164 * t8475 - 2.2140749178833072_f64 * t2239 * t1011 - 4.937333717448355_f64 * t161 * t7962 - 4.937333717448355_f64 * t161 * t7768 + 0.04115066352984959_f64 * t4725 * t8485 + 0.04115066352984959_f64 * t8491 + 0.04115066352984959_f64 * t4715 * t8494 + 0.04115066352984959_f64 * t4725 * t8498 + 0.04115066352984959_f64 * t4725 * t8503 + 1.8805371096875316_f64 * t8506 - 5.40024514194619_f64 * t8508 + 3.600163427964126_f64 * t8510 + 3.600163427964126_f64 * t8512;
    (t8510, t8512, t8514)
}
