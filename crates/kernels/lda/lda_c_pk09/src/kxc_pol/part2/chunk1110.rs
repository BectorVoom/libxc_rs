//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 1110/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk1110<F: Float>(t11121: F, t11159: F, t11201: F, t11235: F, t11261: F, t11297: F, t11335: F, t11375: F, t11432: F, t11487: F, t11994: F, t12036: F, t12119: F, t12169: F, t12225: F, t12289: F) -> F {
    let t12293 = t11121 + t11159 + t11201 + t11235 + t11261 + t11297 + t11335 + t11375 + t11432 + t11487 + t11994 + t12036 + t12119 + t12169 + t12225 + t12289;
    t12293
}
