//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 442/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk442<F: Float>(t168: F, t2152: F, t96: F, t1029: F, t1030: F, t1034: F, t1035: F, t2159: F, t2163: F, t2167: F, t2171: F, t2175: F, t2179: F) -> (F, F, F) {
    let t2318 = t2152 * t168;
    let t2319 = t96 * t2318;
    let t2335 = t1029 + t1030 + F::new(4.59690841536205) * t2159 + F::new(4.59690841536205) * t2163 - F::new(4.59690841536205) * t2167 + t1034 + t1035 + F::new(0.3056501876701794) * t2171 + F::new(0.3056501876701794) * t2175 - F::new(0.3056501876701794) * t2179;
    (t2318, t2319, t2335)
}
