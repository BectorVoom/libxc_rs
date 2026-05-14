//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 946/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk946<F: Float>(t11714: F, t7292: F, t2016: F, t309: F, t454: F, t2812: F, t7300: F, t2042: F, t1905: F, t7704: F, t10954: F, t10959: F, t10962: F, t10966: F, t11062: F, t11066: F, t11070: F, t11073: F, t11076: F, t6323: F, t6327: F, t6337: F, t6467: F, t6747: F, t6749: F, t6755: F) -> (F, F, F, F, F, F) {
    let t11715 = t11714 * t7292;
    let t11717 = t309 * t454 * t2016;
    let t11720 = t2812 * t7300;
    let t11721 = t11720 * t2042;
    let t11723 = t2812 * t7292;
    let t11733 = t309 * t1905 * t7704;
    let t11749 = t6747 - 11.879313099038017 * t6323 + t6749 + 11.879313099038017 * t6327 - 11.879313099038017 * t10954 + 23.758626198076033 * t10959 - 3.959771033012672 * t10962 - 11.879313099038017 * t10966 - 11.879313099038017 * t11062 - 3.959771033012672 * t6337 - t6755 + 3.959771033012672 * t6467 + 11.879313099038017 * t11066 - 11.879313099038017 * t11070 + 3.959771033012672 * t11073 + 11.879313099038017 * t11076;
    (t11715, t11717, t11721, t11723, t11733, t11749)
}
