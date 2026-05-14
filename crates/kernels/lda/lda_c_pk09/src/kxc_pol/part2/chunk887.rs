//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 887/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk887<F: Float>(t51: F, t2: F, t2146: F, t258: F, t630: F, t2732: F, t6322: F, zeta_threshold: F) -> (F, F) {
    let t52 = t51 <= zeta_threshold;
    let t10937 = t51 * t2;
    let t10941 = piecewise3(t52, 0.0, -4.0 * t10937 * t258 + 2.0 * t2146 * t630);
    let t10954 = t6322 * t2732;
    (t10941, t10954)
}
