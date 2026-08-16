//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 517/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk517(t17: f64, t217: f64, t68: f64, t1142: f64, t1146: f64, t13: f64, t21: f64, t229: f64, t30: f64, t3039: f64, t3040: f64, t3041: f64, t3044: f64, t3051: f64, t3057: f64, t3061: f64, t3067: f64, t3072: f64, t583: f64, t595: f64, t600: f64, t606: f64, t9: f64) -> (f64, f64) {
    let t3079 = 1.0_f64 / t217 / t17;
    let t3080 = t3079 * t68;
    let t3086 = -0.01150566515902319_f64 * t3039 * t3041 + 0.2761359638165566_f64 * t583 * t3044 * t21 - 0.01926747391157974_f64 * t3051 * t3041 + 0.1855079159154325_f64 * t3057 * t229 * t1146 - 0.24734388788724335_f64 * t595 * t3061 + (1e-21_f64 as f64) * t3067 * t1146 * t30 * t9 - 0.4023321660824716_f64 * t600 * t3072 + 0.008689181157057755_f64 * t13 * t1142 * t30 - 0.2587367761938941_f64 * t3080 * t3040 * t30 + 0.3881051642908412_f64 * t606 * t3072;
    (t3080, t3086)
}
