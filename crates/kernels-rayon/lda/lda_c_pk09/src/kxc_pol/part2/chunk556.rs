//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 556/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk556(t158: f64, t10: f64, t733: f64, t93: f64, t169: f64, t3161: f64, t96: f64, t3118: f64, t841: f64, t155: f64, t3230: f64, t3233: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3516 = t158 * t158;
    let t3517 = 1.0_f64 / t3516;
    let t3522 = t733 * t10;
    let t3523 = t3522 * t93;
    let t3525 = t96 * t169 * t3161;
    let t3527 = 0.08230132705969918_f64 * t3523 * t3525;
    let t3529 = 0.05486755137313279_f64 * t3118 * t841;
    let t3534 = t155 * t3230;
    let t3536 = t155 * t3233;
    (t3517, t3522, t3523, t3527, t3529, t3534, t3536)
}
