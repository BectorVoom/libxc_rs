//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 1077/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk1077(t10954: f64, t10959: f64, t10962: f64, t10966: f64, t11062: f64, t11066: f64, t11070: f64, t11073: f64, t11076: f64, t6323: f64, t6327: f64, t6337: f64, t6467: f64, t6747: f64, t6749: f64, t6755: f64) -> f64 {
    let t11749 = t6747 - 11.879313099038017_f64 * t6323 + t6749 + 11.879313099038017_f64 * t6327 - 11.879313099038017_f64 * t10954 + 23.758626198076033_f64 * t10959 - 3.959771033012672_f64 * t10962 - 11.879313099038017_f64 * t10966 - 11.879313099038017_f64 * t11062 - 3.959771033012672_f64 * t6337 - t6755 + 3.959771033012672_f64 * t6467 + 11.879313099038017_f64 * t11066 - 11.879313099038017_f64 * t11070 + 3.959771033012672_f64 * t11073 + 11.879313099038017_f64 * t11076;
    t11749
}
