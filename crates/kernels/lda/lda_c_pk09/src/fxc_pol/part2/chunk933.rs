//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 933/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk933<F: Float>(t1348: F, t1406: F, t1476: F, t1481: F, t2517: F, t311: F, t9770: F, t9827: F, t9830: F, t9833: F, t9837: F, t9840: F, t9843: F, t9847: F, t9851: F, t9854: F, t9857: F, t9860: F, t9863: F, t9865: F, t9867: F, t9870: F, t9874: F, t9878: F) -> F {
    let t9881 = F::cast_from(19.489173774580152_f64) * t9827 * t311 + F::cast_from(1.8805371096875316_f64) * t9830 * t311 - F::cast_from(3.7610742193750633_f64) * t9833 * t311 + F::cast_from(6.496391258193384_f64) * t9837 + F::cast_from(2.2140749178833072_f64) * t9840 + F::cast_from(7.108175748183851_f64) * t1481 * t9843 - F::cast_from(7.108175748183851_f64) * t9847 * t2517 - F::cast_from(7.108175748183851_f64) * t1476 * t9851 + F::cast_from(6.211752672544321_f64) * t9854 + F::cast_from(0.013716887843283197_f64) * t9857 + F::cast_from(1.1846959580306418_f64) * t9860 - F::cast_from(4.738783832122567_f64) * t9863 - F::cast_from(0.8091720650647759_f64) * t9865 - F::cast_from(1.6457779058161184_f64) * t9867 - F::cast_from(0.013716887843283197_f64) * t9870 + F::cast_from(2.2140749178833072_f64) * t1406 * t9770 - F::cast_from(2.2140749178833072_f64) * t9874 * t311 + F::cast_from(0.04115066352984959_f64) * t1348 * t9878;
    t9881
}
