//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 818/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk818(t702: f64, t8234: f64, t89: f64, t179: f64, t192: f64, t3729: f64, t3734: f64, t3736: f64, t3768: f64, t3773: f64, t7706: f64, t7768: f64, t7776: f64, t7962: f64, t8049: f64, t98: f64) -> f64 {
    let t8235 = t8234 * t702;
    let t8236 = t8235 * t89;
    let t8257 = -2.2140749178833072_f64 * t8236 * t98 - 18.635258017632964_f64 * t179 * t7776 - 18.635258017632964_f64 * t179 * t7706 + 2.2140749178833072_f64 * t192 * t7962 + 2.2140749178833072_f64 * t192 * t7768 - 18.635258017632964_f64 * t179 * t7962 - 18.635258017632964_f64 * t179 * t7768 + 18.635258017632964_f64 * t179 * t8049 + t3729 - 0.027433775686566395_f64 * t3734 - 0.027433775686566395_f64 * t3736 + 4.738783832122567_f64 * t3768 - 2.427516195194328_f64 * t3773;
    t8257
}
