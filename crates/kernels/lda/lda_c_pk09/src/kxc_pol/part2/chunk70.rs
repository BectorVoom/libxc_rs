//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 70/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk70<F: Float>(t169: F, t96: F, t77: F, t88: F, t142: F) -> (F, F, F, F, F) {
    let t170 = t96 * t169;
    let t174 = F::new(1.5625) * t77 + F::new(0.3208669506079574);
    let t177 = f64::atan(F::new(0.16004110557090126) / t174);
    let t178 = t177 * t88;
    let t179 = t178 * t142;
    (t170, t174, t177, t178, t179)
}
