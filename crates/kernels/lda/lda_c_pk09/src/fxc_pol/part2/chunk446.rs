//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 446/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk446<F: Float>(t2314: F, t80: F, t1094: F, t1014: F, t1015: F, t1019: F, t1020: F, t2159: F, t2163: F, t2167: F, t2171: F, t2175: F, t2179: F) -> (F, F, F) {
    let t2362 = t2314 * t80;
    let t2363 = t2362 * t1094;
    let t2378 = t1014 + t1015 + F::new(4.431130547644593) * t2159 + F::new(4.431130547644593) * t2163 - F::new(4.431130547644593) * t2167 + t1019 + t1020 + F::new(0.2946275542389858) * t2171 + F::new(0.2946275542389858) * t2175 - F::new(0.2946275542389858) * t2179;
    (t2362, t2363, t2378)
}
