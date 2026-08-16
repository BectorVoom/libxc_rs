//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 1103/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk1103(t11586: f64, t476: f64, t11589: f64, t333: f64, t1672: f64, t2903: f64, t11248: f64, t1901: f64, t10959: f64, t11066: f64, t11073: f64, t11076: f64, t11529: f64, t11532: f64, t11535: f64, t11539: f64, t11542: f64, t6323: f64, t6337: f64, t6467: f64, t6502: f64, t6506: f64, t6508: f64, t6523: f64, t6550: f64) -> (f64, f64, f64, f64, f64) {
    let t12174 = t11586 * t476;
    let t12175 = t333 * t11589;
    let t12185 = t2903 * t1672;
    let t12187 = t1901 * t11248;
    let t12203 = 0.64_f64 * t11066 + 1.28_f64 * t10959 + 6.416968383055361_f64 * t11529 - 6.416968383055361_f64 * t11532 - 6.416968383055361_f64 * t11535 + 9.625452574583042_f64 * t11539 - 6.416968383055361_f64 * t11542 + 0.64_f64 * t11076 + t6502 + 0.21333333333333335_f64 * t11073 + t6523 - 0.21333333333333335_f64 * t6337 - 0.64_f64 * t6323 + 2.1389894610184537_f64 * t6550 + t6506 - 2.1389894610184537_f64 * t6508 + 0.21333333333333335_f64 * t6467;
    (t12174, t12175, t12185, t12187, t12203)
}
