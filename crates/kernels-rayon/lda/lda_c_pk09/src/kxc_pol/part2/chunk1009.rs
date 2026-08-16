//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 1009/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk1009(t56: f64, t6329: f64, t1729: f64, t2730: f64, t59: f64) -> (f64, f64) {
    let t10956 = t6329 * t56;
    let t10957 = t2730 * t1729;
    let t10958 = t59 * t10957;
    let t10959 = t10956 * t10958;
    (t10957, t10959)
}
