//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 785/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk785<F: Float>(t3323: F, t3326: F, t3424: F, t3426: F, t3428: F, t4245: F, t4252: F, t4254: F, t7870: F, t7875: F, t7879: F, t7884: F, t7888: F, t7896: F, t7900: F, t7904: F, t7908: F, t7913: F, t7917: F, t7919: F, t7923: F, t7926: F, t7928: F, t7931: F, t7935: F, t7939: F, t7942: F) -> (F, F) {
    let t9375 = 0.2037667917801196 * t3323 + 0.2037667917801196 * t3326 + t4245 + 9.1938168307241 * t7870 - 9.1938168307241 * t7875 + 9.1938168307241 * t7879 - 9.1938168307241 * t7884 + 9.1938168307241 * t7888 + 6.129211220482733 * t3424 + 6.129211220482733 * t3426 - 6.129211220482733 * t3428 + t4252 + t4254;
    let t9390 = 0.6113003753403587 * t7896 - 9.1938168307241 * t7900 - 9.1938168307241 * t7904 - 9.1938168307241 * t7908 + 13.790725246086149 * t7913 + 9.1938168307241 * t7917 + 0.3056501876701794 * t7919 + 0.3056501876701794 * t7923 + 0.3056501876701794 * t7926 + 0.3056501876701794 * t7928 + 0.3056501876701794 * t7931 + 0.3056501876701794 * t7935 + 0.2037667917801196 * t7939 + 0.2037667917801196 * t7942;
    (t9375, t9390)
}
