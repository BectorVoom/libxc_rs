//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 847/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk847<F: Float>(t3319: F, t3323: F, t3326: F, t3331: F, t7896: F, t7919: F, t7923: F, t7926: F, t7928: F, t7931: F, t7935: F, t7939: F, t7942: F) -> F {
    let t8718 = F::new(11.879313099038017) * t3319 + F::new(7.919542066025344) * t3323 + F::new(7.919542066025344) * t3326 + t3331 + F::new(23.758626198076033) * t7896 + F::new(11.879313099038017) * t7919 + F::new(11.879313099038017) * t7923 + F::new(11.879313099038017) * t7926 + F::new(11.879313099038017) * t7928 + F::new(11.879313099038017) * t7931 + F::new(11.879313099038017) * t7935 + F::new(7.919542066025344) * t7939 + F::new(7.919542066025344) * t7942;
    t8718
}
