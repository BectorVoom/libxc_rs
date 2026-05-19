//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 798/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk798<F: Float>(t3319: F, t3323: F, t3326: F, t3870: F, t7896: F, t7919: F, t7923: F, t7926: F, t7928: F, t7931: F, t7935: F, t7939: F, t7942: F) -> F {
    let t8026 = F::new(1.5625) * t3319 + F::cast_from(1.0416666666666667_f64) * t3323 + F::cast_from(1.0416666666666667_f64) * t3326 + t3870 + F::new(3.125) * t7896 + F::new(1.5625) * t7919 + F::new(1.5625) * t7923 + F::new(1.5625) * t7926 + F::new(1.5625) * t7928 + F::new(1.5625) * t7931 + F::new(1.5625) * t7935 + F::cast_from(1.0416666666666667_f64) * t7939 + F::cast_from(1.0416666666666667_f64) * t7942;
    t8026
}
