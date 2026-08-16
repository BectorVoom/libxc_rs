//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 847/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk847(t3319: f64, t3323: f64, t3326: f64, t3331: f64, t7896: f64, t7919: f64, t7923: f64, t7926: f64, t7928: f64, t7931: f64, t7935: f64, t7939: f64, t7942: f64) -> f64 {
    let t8718 = 11.879313099038017_f64 * t3319 + 7.919542066025344_f64 * t3323 + 7.919542066025344_f64 * t3326 + t3331 + 23.758626198076033_f64 * t7896 + 11.879313099038017_f64 * t7919 + 11.879313099038017_f64 * t7923 + 11.879313099038017_f64 * t7926 + 11.879313099038017_f64 * t7928 + 11.879313099038017_f64 * t7931 + 11.879313099038017_f64 * t7935 + 7.919542066025344_f64 * t7939 + 7.919542066025344_f64 * t7942;
    t8718
}
