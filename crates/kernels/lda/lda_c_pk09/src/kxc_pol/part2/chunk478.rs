//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 478/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk478<F: Float>(t1363: F, t2648: F, t310: F, t1244: F, t1256: F, t1264: F, t1273: F, t2502: F, t2505: F, t2542: F, t2546: F, t1278: F) -> (F, F, F, F) {
    let t2649 = t2648 * t1363;
    let t2650 = t310 * t2649;
    let t2665 = t1244 - F::cast_from(3.2084841915276807_f64) * t2542 + t1256 + F::cast_from(3.2084841915276807_f64) * t2546 + t1264 - F::new(0.64) * t2502 + t1273 + F::new(0.64) * t2505;
    let t2666 = t2665 * t1278;
    (t2649, t2650, t2665, t2666)
}
