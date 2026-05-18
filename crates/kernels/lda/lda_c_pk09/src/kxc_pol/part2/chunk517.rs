//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 517/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk517<F: Float>(t17: F, t217: F, t68: F, t1142: F, t1146: F, t13: F, t21: F, t229: F, t30: F, t3039: F, t3040: F, t3041: F, t3044: F, t3051: F, t3057: F, t3061: F, t3067: F, t3072: F, t583: F, t595: F, t600: F, t606: F, t9: F) -> (F, F) {
    let t3079 = F::new(1.0) / t217 / t17;
    let t3080 = t3079 * t68;
    let t3086 = -F::new(0.01150566515902319) * t3039 * t3041 + F::new(0.2761359638165566) * t583 * t3044 * t21 - F::new(0.01926747391157974) * t3051 * t3041 + F::new(0.1855079159154325) * t3057 * t229 * t1146 - F::new(0.24734388788724335) * t595 * t3061 + 1e-21 * t3067 * t1146 * t30 * t9 - F::new(0.4023321660824716) * t600 * t3072 + F::new(0.008689181157057755) * t13 * t1142 * t30 - F::new(0.2587367761938941) * t3080 * t3040 * t30 + F::new(0.3881051642908412) * t606 * t3072;
    (t3080, t3086)
}
