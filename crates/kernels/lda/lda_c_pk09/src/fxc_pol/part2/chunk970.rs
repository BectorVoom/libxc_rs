//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 970/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk970<F: Float>(t11586: F, t476: F, t11589: F, t333: F, t1672: F, t2903: F, t11248: F, t1901: F, t10959: F, t11066: F, t11073: F, t11076: F, t11529: F, t11532: F, t11535: F, t11539: F, t11542: F, t6323: F, t6337: F, t6467: F, t6502: F, t6506: F, t6508: F, t6523: F, t6550: F) -> (F, F, F, F, F) {
    let t12174 = t11586 * t476;
    let t12175 = t333 * t11589;
    let t12185 = t2903 * t1672;
    let t12187 = t1901 * t11248;
    let t12203 = 0.64 * t11066 + 1.28 * t10959 + 6.416968383055361 * t11529 - 6.416968383055361 * t11532 - 6.416968383055361 * t11535 + 9.625452574583042 * t11539 - 6.416968383055361 * t11542 + 0.64 * t11076 + t6502 + 0.21333333333333335 * t11073 + t6523 - 0.21333333333333335 * t6337 - 0.64 * t6323 + 2.1389894610184537 * t6550 + t6506 - 2.1389894610184537 * t6508 + 0.21333333333333335 * t6467;
    (t12174, t12175, t12185, t12187, t12203)
}
