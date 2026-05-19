//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 818/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk818<F: Float>(t702: F, t8234: F, t89: F, t179: F, t192: F, t3729: F, t3734: F, t3736: F, t3768: F, t3773: F, t7706: F, t7768: F, t7776: F, t7962: F, t8049: F, t98: F) -> F {
    let t8235 = t8234 * t702;
    let t8236 = t8235 * t89;
    let t8257 = -F::cast_from(2.2140749178833072_f64) * t8236 * t98 - F::cast_from(18.635258017632964_f64) * t179 * t7776 - F::cast_from(18.635258017632964_f64) * t179 * t7706 + F::cast_from(2.2140749178833072_f64) * t192 * t7962 + F::cast_from(2.2140749178833072_f64) * t192 * t7768 - F::cast_from(18.635258017632964_f64) * t179 * t7962 - F::cast_from(18.635258017632964_f64) * t179 * t7768 + F::cast_from(18.635258017632964_f64) * t179 * t8049 + t3729 - F::cast_from(0.027433775686566395_f64) * t3734 - F::cast_from(0.027433775686566395_f64) * t3736 + F::cast_from(4.738783832122567_f64) * t3768 - F::cast_from(2.427516195194328_f64) * t3773;
    t8257
}
