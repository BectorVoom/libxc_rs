//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 933/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk933<F: Float>(t1348: F, t1406: F, t1476: F, t1481: F, t2517: F, t311: F, t9770: F, t9827: F, t9830: F, t9833: F, t9837: F, t9840: F, t9843: F, t9847: F, t9851: F, t9854: F, t9857: F, t9860: F, t9863: F, t9865: F, t9867: F, t9870: F, t9874: F, t9878: F) -> F {
    let t9881 = F::new(19.489173774580152) * t9827 * t311 + F::new(1.8805371096875316) * t9830 * t311 - F::new(3.7610742193750633) * t9833 * t311 + F::new(6.496391258193384) * t9837 + F::new(2.2140749178833072) * t9840 + F::new(7.108175748183851) * t1481 * t9843 - F::new(7.108175748183851) * t9847 * t2517 - F::new(7.108175748183851) * t1476 * t9851 + F::new(6.211752672544321) * t9854 + F::new(0.013716887843283197) * t9857 + F::new(1.1846959580306418) * t9860 - F::new(4.738783832122567) * t9863 - F::new(0.8091720650647759) * t9865 - F::new(1.6457779058161184) * t9867 - F::new(0.013716887843283197) * t9870 + F::new(2.2140749178833072) * t1406 * t9770 - F::new(2.2140749178833072) * t9874 * t311 + F::new(0.04115066352984959) * t1348 * t9878;
    t9881
}
