//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 933/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk933(t1348: f64, t1406: f64, t1476: f64, t1481: f64, t2517: f64, t311: f64, t9770: f64, t9827: f64, t9830: f64, t9833: f64, t9837: f64, t9840: f64, t9843: f64, t9847: f64, t9851: f64, t9854: f64, t9857: f64, t9860: f64, t9863: f64, t9865: f64, t9867: f64, t9870: f64, t9874: f64, t9878: f64) -> f64 {
    let t9881 = 19.489173774580152_f64 * t9827 * t311 + 1.8805371096875316_f64 * t9830 * t311 - 3.7610742193750633_f64 * t9833 * t311 + 6.496391258193384_f64 * t9837 + 2.2140749178833072_f64 * t9840 + 7.108175748183851_f64 * t1481 * t9843 - 7.108175748183851_f64 * t9847 * t2517 - 7.108175748183851_f64 * t1476 * t9851 + 6.211752672544321_f64 * t9854 + 0.013716887843283197_f64 * t9857 + 1.1846959580306418_f64 * t9860 - 4.738783832122567_f64 * t9863 - 0.8091720650647759_f64 * t9865 - 1.6457779058161184_f64 * t9867 - 0.013716887843283197_f64 * t9870 + 2.2140749178833072_f64 * t1406 * t9770 - 2.2140749178833072_f64 * t9874 * t311 + 0.04115066352984959_f64 * t1348 * t9878;
    t9881
}
