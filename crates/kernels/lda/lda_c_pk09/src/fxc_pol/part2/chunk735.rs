//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 735/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk735<F: Float>(t3323: F, t3326: F, t3424: F, t3426: F, t3428: F, t3803: F, t3810: F, t3812: F, t7870: F, t7875: F, t7879: F, t7884: F, t7888: F, t7896: F, t7900: F, t7904: F, t7908: F, t7913: F, t7917: F, t7919: F, t7923: F, t7926: F, t7928: F, t7931: F, t7935: F, t7939: F, t7942: F) -> (F, F) {
    let t8298 = 0.03016988933062603 * t3323 + 0.03016988933062603 * t3326 + t3803 + 1.3612445574954364 * t7870 - 1.3612445574954364 * t7875 + 1.3612445574954364 * t7879 - 1.3612445574954364 * t7884 + 1.3612445574954364 * t7888 + 0.9074963716636242 * t3424 + 0.9074963716636242 * t3426 - 0.9074963716636242 * t3428 + t3810 + t3812;
    let t8313 = 0.09050966799187808 * t7896 - 1.3612445574954364 * t7900 - 1.3612445574954364 * t7904 - 1.3612445574954364 * t7908 + 2.0418668362431545 * t7913 + 1.3612445574954364 * t7917 + 0.04525483399593904 * t7919 + 0.04525483399593904 * t7923 + 0.04525483399593904 * t7926 + 0.04525483399593904 * t7928 + 0.04525483399593904 * t7931 + 0.04525483399593904 * t7935 + 0.03016988933062603 * t7939 + 0.03016988933062603 * t7942;
    (t8298, t8313)
}
