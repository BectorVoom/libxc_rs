//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 39/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk39<F: Float>(t66: F, t77: F, t83: F, t34: F) -> (F, F, F) {
    let t86 = F::new(2.2155652738222966) * t66 + F::new(0.2946275542389858) * t77 + F::new(0.0346182074034769);
    let t87 = t83 * t86;
    let t88 = F::new(1.0) / t34;
    (t86, t87, t88)
}
