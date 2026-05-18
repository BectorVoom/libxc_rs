//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 1085/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk1085<F: Float>(t10954: F, t10962: F, t10966: F, t11062: F, t11070: F, t11556: F, t11559: F, t11563: F, t11566: F, t11574: F, t6327: F, t6519: F, t6527: F, t7116: F, t7123: F, t7124: F, t7129: F) -> F {
    let t11897 = F::new(3.0646056102413666) * t6527 - F::new(3.0646056102413666) * t6519 - F::new(1.5323028051206833) * t11556 + F::new(1.5323028051206833) * t11559 - F::new(0.1018833958900598) * t10962 + F::new(1.0215352034137888) * t11563 - F::new(1.0215352034137888) * t11566 - F::new(0.3056501876701794) * t11070 - F::new(0.3056501876701794) * t10954 - F::new(0.3056501876701794) * t10966 - F::new(0.3056501876701794) * t11062 + F::new(1.5323028051206833) * t11574 + F::new(0.3056501876701794) * t6327 + t7116 + t7123 - t7124 - t7129;
    t11897
}
