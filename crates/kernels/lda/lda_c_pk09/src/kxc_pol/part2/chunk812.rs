//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 812/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk812<F: Float>(t3319: F, t3323: F, t3326: F, t3917: F, t7896: F, t7919: F, t7923: F, t7926: F, t7928: F, t7931: F, t7935: F, t7939: F, t7942: F) -> F {
    let t8168 = F::new(0.9421211958699838) * t3319 + F::new(0.6280807972466558) * t3323 + F::new(0.6280807972466558) * t3326 + t3917 + F::new(1.8842423917399675) * t7896 + F::new(0.9421211958699838) * t7919 + F::new(0.9421211958699838) * t7923 + F::new(0.9421211958699838) * t7926 + F::new(0.9421211958699838) * t7928 + F::new(0.9421211958699838) * t7931 + F::new(0.9421211958699838) * t7935 + F::new(0.6280807972466558) * t7939 + F::new(0.6280807972466558) * t7942;
    t8168
}
