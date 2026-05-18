//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 427/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk427<F: Float>(t2159: F, t2163: F, t2167: F, t2171: F, t2175: F, t2179: F, t777: F, t778: F, t782: F, t783: F, t788: F, t89: F) -> (F, F, F) {
    let t2181 = t777 + t778 + F::new(18.75) * t2159 + F::new(18.75) * t2163 - F::new(18.75) * t2167 + t782 + t783 + F::new(1.2466946262544771) * t2171 + F::new(1.2466946262544771) * t2175 - F::new(1.2466946262544771) * t2179;
    let t2182 = t2181 * t788;
    let t2183 = t2182 * t89;
    (t2181, t2182, t2183)
}
