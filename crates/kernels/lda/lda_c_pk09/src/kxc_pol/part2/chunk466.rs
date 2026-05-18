//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 466/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk466<F: Float>(t2143: F, t48: F, t285: F, t284: F, t1584: F, t1586: F, t1588: F, t1590: F, t2502: F, t2505: F, t2542: F, t323: F) -> (F, F, F, F) {
    let t2544 = t48 * t2143;
    let t2545 = t285 * t2544;
    let t2546 = t284 * t2545;
    let t2550 = t1584 - F::new(1.5323028051206833) * t2542 + t1586 + F::new(1.5323028051206833) * t2546 + t1588 - F::new(0.3056501876701794) * t2502 + t1590 + F::new(0.3056501876701794) * t2505;
    let t2551 = t323 * t2550;
    (t2544, t2546, t2550, t2551)
}
