//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 437/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk437<F: Float>(t2262: F, t891: F, t890: F, t2171: F, t2175: F, t2179: F, t856: F, t857: F, t862: F, t89: F, t169: F, t2143: F, t844: F) -> (F, F, F, F, F, F) {
    let t2263 = t891 * t2262;
    let t2264 = t890 * t2263;
    let t2269 = t856 + t857 + F::new(2.0) * t2171 + F::new(2.0) * t2175 - F::new(2.0) * t2179;
    let t2270 = t2269 * t862;
    let t2271 = t2270 * t89;
    let t2275 = t844 * t169 * t2143;
    (t2263, t2264, t2269, t2270, t2271, t2275)
}
