//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 983/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk983<F: Float>(t5043: F, t5047: F, t5056: F, t5071: F, t5126: F, t5128: F, t5134: F, t9623: F, t9628: F, t9631: F, t9635: F, t9742: F, t9746: F, t9750: F, t9753: F, t9756: F) -> F {
    let t10533 = t5126 - F::new(0.9421211958699838) * t5043 + t5128 + F::new(0.9421211958699838) * t5047 - F::new(0.9421211958699838) * t9623 + F::new(1.8842423917399675) * t9628 - F::new(0.3140403986233279) * t9631 - F::new(0.9421211958699838) * t9635 - F::new(0.9421211958699838) * t9742 - F::new(0.3140403986233279) * t5056 - t5134 + F::new(0.3140403986233279) * t5071 + F::new(0.9421211958699838) * t9746 - F::new(0.9421211958699838) * t9750 + F::new(0.3140403986233279) * t9753 + F::new(0.9421211958699838) * t9756;
    t10533
}
