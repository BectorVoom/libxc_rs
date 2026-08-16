//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 901/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk901(t7611: f64, t7658: f64, t7686: f64, t7714: f64, t7736: f64, t7771: f64, t7970: f64, t7997: f64, t8045: f64, t8076: f64, t8088: f64, t8112: f64, t8144: f64, t8257: f64, t8347: f64, t8385: f64, t8419: f64, t8448: f64, t8473: f64, t8514: f64, t8548: f64, t8575: f64, t8599: f64, t8631: f64, t8654: f64, t8677: f64, t8730: f64, t8757: f64, t8848: f64, t8890: f64, t8972: f64, t9554: f64) -> f64 {
    let t9559 = t8575 + t7611 + t7736 + t8548 + t9554 + t8848 + t8677 + t8257 + t7970 + t8088 + t7658 + t8890 + t8144 + t7686 + t8385 + t8419 + t8112 + t8631 + t8599 + t7997 + t8045 + t7714 + t8473 + t8654 + t8972 + t8757 + t8448 + t8347 + t8730 + t8514 + t7771 + t8076;
    t9559
}
