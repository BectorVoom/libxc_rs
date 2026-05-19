//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 749/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk749<F: Float>(t44: F, t258: F, t620: F, t7759: F, t7762: F, zeta_threshold: F) -> F {
    let t45 = t44 <= zeta_threshold;
    let t7766 = piecewise3::<F>(t45, F::new(0.0), -F::new(2.0) / F::new(9.0) * t7759 * t620 + F::new(2.0) / F::new(3.0) * t7762 * t258);
    t7766
}
