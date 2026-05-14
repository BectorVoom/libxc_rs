//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 418/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk418<F: Float>(t1081: F, t1082: F, t1086: F, t1087: F, t2159: F, t2163: F, t2167: F, t2171: F, t2175: F, t2179: F) -> (F,) {
    let t2314 = t1081 + t1082 + 0.6806222787477182 * t2159 + 0.6806222787477182 * t2163 - 0.6806222787477182 * t2167 + t1086 + t1087 + 0.04525483399593904 * t2171 + 0.04525483399593904 * t2175 - 0.04525483399593904 * t2179;
    (t2314,)
}
