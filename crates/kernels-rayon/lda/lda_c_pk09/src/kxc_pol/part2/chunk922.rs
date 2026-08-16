//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 922/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk922(t5043: f64, t5047: f64, t5056: f64, t5071: f64, t5731: f64, t5733: f64, t5739: f64, t9623: f64, t9628: f64, t9631: f64, t9635: f64, t9742: f64, t9746: f64, t9750: f64, t9753: f64, t9756: f64) -> f64 {
    let t9758 = t5731 - 11.879313099038017_f64 * t5043 + t5733 + 11.879313099038017_f64 * t5047 - 11.879313099038017_f64 * t9623 + 23.758626198076033_f64 * t9628 - 3.959771033012672_f64 * t9631 - 11.879313099038017_f64 * t9635 - 11.879313099038017_f64 * t9742 - 3.959771033012672_f64 * t5056 - t5739 + 3.959771033012672_f64 * t5071 + 11.879313099038017_f64 * t9746 - 11.879313099038017_f64 * t9750 + 3.959771033012672_f64 * t9753 + 11.879313099038017_f64 * t9756;
    t9758
}
