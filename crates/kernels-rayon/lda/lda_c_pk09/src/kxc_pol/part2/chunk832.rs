//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 832/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk832(t3772: f64, t7731: f64, t169: f64, t7766: f64, t844: f64, t2149: f64, t849: f64, t873: f64, t164: f64, t2210: f64, t3750: f64, t3753: f64, t4034: f64, t4040: f64, t4042: f64, t4065: f64, t4067: f64, t4070: f64, t4077: f64, t7598: f64, t7602: f64) -> f64 {
    let t8453 = t3772 * t7731;
    let t8466 = t844 * t169 * t7766;
    let t8470 = t849 * t873 * t2149;
    let t8473 = -0.04115066352984959_f64 * t4034 - 0.04115066352984959_f64 * t4040 + 4.937333717448355_f64 * t4042 - 2.427516195194328_f64 * t8453 - 4.855032390388656_f64 * t3750 * t7598 - 2.427516195194328_f64 * t3750 * t7602 + 2.427516195194328_f64 * t3753 * t2210 - 2.2140749178833072_f64 * t4065 - 2.2140749178833072_f64 * t4067 + 2.2140749178833072_f64 * t4070 + 1.9882715304939877_f64 * t4077 + 0.04115066352984959_f64 * t164 * t8466 + 0.04115066352984959_f64 * t164 * t8470;
    t8473
}
