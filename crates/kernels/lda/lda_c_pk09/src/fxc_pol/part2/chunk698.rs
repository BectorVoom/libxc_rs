//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 698/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk698<F: Float>(t3323: F, t3326: F, t3421: F, t3424: F, t3426: F, t3428: F, t3445: F, t3454: F, t7870: F, t7875: F, t7879: F, t7884: F, t7888: F, t2165: F, t650: F, t639: F) -> (F, F) {
    let t7893 = 0.8311297508363181 * t3323 + 0.8311297508363181 * t3326 + t3421 + 37.5 * t7870 - 37.5 * t7875 + 37.5 * t7879 - 37.5 * t7884 + 37.5 * t7888 + 25.0 * t3424 + 25.0 * t3426 - 25.0 * t3428 + t3445 + t3454;
    let t7894 = t2165 * t650;
    let t7895 = t639 * t7894;
    (t7893, t7895)
}
