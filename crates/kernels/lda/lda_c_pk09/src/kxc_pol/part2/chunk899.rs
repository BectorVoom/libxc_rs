//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 899/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk899<F: Float>(t8998: F, t9018: F, t9032: F, t9069: F, t9153: F, t9181: F, t9217: F, t9237: F, t9256: F, t9335: F, t9417: F, t9440: F, t9467: F, t9487: F, t9511: F, t9539: F) -> F {
    let t9543 = t8998 + t9018 + t9032 + t9069 + t9153 + t9181 + t9217 + t9237 + t9256 + t9335 + t9417 + t9440 + t9467 + t9487 + t9511 + t9539;
    t9543
}
