//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 1074/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk1074(t11679: f64, t471: f64, t2042: f64, t10954: f64, t10966: f64, t11062: f64, t11070: f64, t11464: f64, t11467: f64, t11470: f64, t11673: f64, t2032: f64, t2813: f64, t2826: f64, t6288: f64, t7241: f64, t7244: f64, t7253: f64, t7256: f64, t7276: f64, t7279: f64, t7297: f64, t7302: f64, t7310: f64) -> f64 {
    let t11680 = t471 * t11679;
    let t11681 = t11680 * t2042;
    let t11688 = 0.10237773105191754_f64 * t11070 + 0.10237773105191754_f64 * t10954 + 0.10237773105191754_f64 * t10966 + 0.10237773105191754_f64 * t11062 + 0.04991874779241519_f64 * t11464 + 0.02466859483068398_f64 * t11467 - 0.02466859483068398_f64 * t11470 + t7241 / 6.0_f64 + t7244 / 6.0_f64 + t7253 / 6.0_f64 - t7256 / 6.0_f64 - t11673 / 6.0_f64 - t7276 / 12.0_f64 - t7279 / 6.0_f64 - t2826 * t2032 / 6.0_f64 - t11681 / 6.0_f64 - t2813 * t6288 / 6.0_f64 - t7297 / 6.0_f64 + t7302 / 6.0_f64 + t7310 / 6.0_f64;
    t11688
}
