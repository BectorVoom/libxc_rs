//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 1077/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk1077<F: Float>(t10954: F, t10959: F, t10962: F, t10966: F, t11062: F, t11066: F, t11070: F, t11073: F, t11076: F, t6323: F, t6327: F, t6337: F, t6467: F, t6747: F, t6749: F, t6755: F) -> F {
    let t11749 = t6747 - F::new(11.879313099038017) * t6323 + t6749 + F::new(11.879313099038017) * t6327 - F::new(11.879313099038017) * t10954 + F::new(23.758626198076033) * t10959 - F::new(3.959771033012672) * t10962 - F::new(11.879313099038017) * t10966 - F::new(11.879313099038017) * t11062 - F::new(3.959771033012672) * t6337 - t6755 + F::new(3.959771033012672) * t6467 + F::new(11.879313099038017) * t11066 - F::new(11.879313099038017) * t11070 + F::new(3.959771033012672) * t11073 + F::new(11.879313099038017) * t11076;
    t11749
}
